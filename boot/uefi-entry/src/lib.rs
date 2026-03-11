#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::ffi::c_void;

const COM1_PORT: u16 = 0x3F8;
const LINE_STATUS_DATA_READY: u8 = 1;
const LINE_STATUS_TRANSMITTER_EMPTY: u8 = 1 << 5;

/// UEFI status code.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct EfiStatus(pub usize);

impl EfiStatus {
    /// Successful UEFI status result.
    pub const SUCCESS: Self = Self(0);
}

/// Opaque UEFI image handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct EfiHandle(pub *mut c_void);

/// Opaque UEFI system table pointer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct EfiSystemTable(pub *mut c_void);

/// Minimal COM1 serial port writer for early boot diagnostics.
struct SerialCom1;

impl SerialCom1 {
    const fn new() -> Self {
        Self
    }

    fn write_byte(&mut self, byte: u8) {
        while !self.transmitter_empty() {}
        port_write_u8(COM1_PORT, byte);
    }

    fn write_all(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
    }

    fn transmitter_empty(&self) -> bool {
        let status = port_read_u8(COM1_PORT + 5);
        (status & LINE_STATUS_TRANSMITTER_EMPTY) != 0
    }
}

#[cfg(target_arch = "x86_64")]
fn port_write_u8(port: u16, value: u8) {
    // SAFETY: This emits an x86_64 `out` instruction to a caller-provided I/O port and does not
    // violate Rust aliasing rules; callers restrict usage to early-boot COM1 diagnostics.
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

#[cfg(target_arch = "x86_64")]
fn port_read_u8(port: u16) -> u8 {
    let mut value: u8;
    // SAFETY: This emits an x86_64 `in` instruction from a caller-provided I/O port and stores
    // the result in a local register-backed byte.
    unsafe {
        core::arch::asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
    value
}

#[cfg(not(target_arch = "x86_64"))]
fn port_write_u8(_port: u16, _value: u8) {}

#[cfg(not(target_arch = "x86_64"))]
fn port_read_u8(_port: u16) -> u8 {
    0
}

/// Returns the deterministic kernel message expected by boot milestone consumers.
#[must_use]
pub const fn kernel_entry_message() -> &'static [u8] {
    kernel::boot_banner_bytes()
}

/// UEFI ABI entrypoint for the boot milestone.
///
/// Writes the canonical kernel entry banner to COM1 as the first concrete firmware output path.
#[no_mangle]
pub extern "efiapi" fn efi_main(_image: EfiHandle, _system_table: EfiSystemTable) -> EfiStatus {
    let mut serial = SerialCom1::new();
    serial.write_all(kernel_entry_message());
    serial.write_all(b"\r\n");
    EfiStatus::SUCCESS
}

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    let mut serial = SerialCom1::new();
    serial.write_all(b"tosm-os: panic in uefi-entry\r\n");
    loop {}
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::{kernel_entry_message, EfiStatus, LINE_STATUS_DATA_READY};

    #[test]
    fn entry_message_matches_kernel_banner() {
        assert_eq!(kernel_entry_message(), b"tosm-os: kernel entry reached");
    }

    #[test]
    fn efi_status_success_value_is_zero() {
        assert_eq!(EfiStatus::SUCCESS.0, 0);
    }

    #[test]
    fn line_status_data_ready_bit_is_low_bit() {
        assert_eq!(LINE_STATUS_DATA_READY, 1);
    }
}
