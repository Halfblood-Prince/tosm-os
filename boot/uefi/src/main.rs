#![no_main]
#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::arch::asm;
use core::ffi::c_void;
use core::fmt::{self, Write};

use kernel::boot_banner;

const COM1_PORT: u16 = 0x3F8;
const EFI_SUCCESS: usize = 0;
const EFI_RESET_SHUTDOWN: usize = 2;

#[repr(C)]
struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
struct EfiRuntimeServices {
    header: EfiTableHeader,
    _reserved_time_services: [usize; 13],
    reset_system: unsafe extern "efiapi" fn(
        reset_type: usize,
        status: usize,
        data_size: usize,
        reset_data: *const u16,
    ) -> !,
}

#[repr(C)]
struct EfiSystemTable {
    header: EfiTableHeader,
    _firmware_vendor: *const u16,
    _firmware_revision: u32,
    _console_in_handle: *mut c_void,
    _con_in: *mut c_void,
    _console_out_handle: *mut c_void,
    _con_out: *mut c_void,
    _standard_error_handle: *mut c_void,
    _std_err: *mut c_void,
    _runtime_services: *mut EfiRuntimeServices,
}

#[unsafe(no_mangle)]
extern "efiapi" fn efi_main(
    _image_handle: *mut c_void,
    system_table: *mut EfiSystemTable,
) -> usize {
    let mut serial = SerialPort::new(COM1_PORT);
    serial.init();
    serial.write_line(boot_banner());

    // Safety: UEFI passes a valid system table pointer to the firmware entrypoint.
    let runtime_services = unsafe { (*system_table)._runtime_services };
    // Safety: `reset_system` is provided by firmware and does not return.
    unsafe {
        ((*runtime_services).reset_system)(EFI_RESET_SHUTDOWN, EFI_SUCCESS, 0, core::ptr::null())
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut serial = SerialPort::new(COM1_PORT);
    serial.init();
    let _ = writeln!(serial, "panic: {info}");

    loop {
        // Safety: halting the current CPU is valid after an unrecoverable panic.
        unsafe {
            asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

struct SerialPort {
    base_port: u16,
}

impl SerialPort {
    const fn new(base_port: u16) -> Self {
        Self { base_port }
    }

    fn init(&mut self) {
        self.write_register(1, 0x00);
        self.write_register(3, 0x80);
        self.write_register(0, 0x03);
        self.write_register(1, 0x00);
        self.write_register(3, 0x03);
        self.write_register(2, 0xC7);
        self.write_register(4, 0x0B);
    }

    fn write_line(&mut self, line: &str) {
        let _ = self.write_str(line);
        let _ = self.write_str("\r\n");
    }

    fn write_register(&self, offset: u16, value: u8) {
        // Safety: writing to the UART I/O ports is confined to the known COM1 range.
        unsafe {
            outb(self.base_port + offset, value);
        }
    }

    fn line_status(&self) -> u8 {
        // Safety: reading from the UART line-status register is confined to the known COM1 range.
        unsafe { inb(self.base_port + 5) }
    }

    fn write_byte(&self, byte: u8) {
        while self.line_status() & 0x20 == 0 {
            core::hint::spin_loop();
        }

        // Safety: writing a byte to the UART transmit register is confined to the known COM1 range.
        unsafe {
            outb(self.base_port, byte);
        }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

unsafe fn outb(port: u16, value: u8) {
    // Safety: caller guarantees the port is valid for byte-sized output.
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;

    // Safety: caller guarantees the port is valid for byte-sized input.
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            lateout("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }

    value
}
