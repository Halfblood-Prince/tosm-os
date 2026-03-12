#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::ffi::c_void;

const COM1_PORT: u16 = 0x3F8;
const LINE_STATUS_TRANSMITTER_EMPTY: u8 = 1 << 5;
const INTERRUPT_ENABLE_OFFSET: u16 = 1;
const FIFO_CONTROL_OFFSET: u16 = 2;
const LINE_CONTROL_OFFSET: u16 = 3;
const MODEM_CONTROL_OFFSET: u16 = 4;
const LINE_STATUS_OFFSET: u16 = 5;

const LINE_CONTROL_DLAB: u8 = 1 << 7;
const LINE_CONTROL_8N1: u8 = 0b0000_0011;
const FIFO_ENABLE_CLEAR_14B: u8 = 0b1100_0111;
const MODEM_CONTROL_DTR_RTS_OUT2: u8 = 0b0000_1011;
const BAUD_DIVISOR_38400: u8 = 3;

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

    fn init(&mut self) {
        // Disable UART interrupts during initialization.
        port_write_u8(COM1_PORT + INTERRUPT_ENABLE_OFFSET, 0x00);
        // Enable DLAB so divisor latch bytes can be configured.
        port_write_u8(COM1_PORT + LINE_CONTROL_OFFSET, LINE_CONTROL_DLAB);
        // Configure 38400 baud divisor (low byte then high byte).
        port_write_u8(COM1_PORT, BAUD_DIVISOR_38400);
        port_write_u8(COM1_PORT + INTERRUPT_ENABLE_OFFSET, 0x00);
        // Clear DLAB and select 8 data bits, no parity, one stop bit.
        port_write_u8(COM1_PORT + LINE_CONTROL_OFFSET, LINE_CONTROL_8N1);
        // Enable FIFO and clear both queues with a conservative trigger level.
        port_write_u8(COM1_PORT + FIFO_CONTROL_OFFSET, FIFO_ENABLE_CLEAR_14B);
        // Assert DTR/RTS/OUT2 for basic transmitter readiness.
        port_write_u8(COM1_PORT + MODEM_CONTROL_OFFSET, MODEM_CONTROL_DTR_RTS_OUT2);
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
        let status = port_read_u8(COM1_PORT + LINE_STATUS_OFFSET);
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

/// Returns the deterministic kernel banner line expected by boot milestone consumers.
#[must_use]
pub const fn kernel_entry_message_line() -> &'static [u8] {
    kernel::boot_banner_line_bytes()
}

/// Returns the deterministic early-boot panic line expected by firmware panic paths.
#[must_use]
pub const fn panic_message_line() -> &'static [u8] {
    kernel::boot_panic_line_bytes()
}

/// Returns the deterministic completion line expected before firmware returns success.
#[must_use]
pub const fn entry_done_message_line() -> &'static [u8] {
    kernel::boot_entry_done_line_bytes()
}

/// Shared boot entry implementation used by the UEFI application entry symbol.
///
/// Writes the canonical kernel entry banner to COM1 as the first concrete firmware output path.
pub fn run_entry(_image: EfiHandle, _system_table: EfiSystemTable) -> EfiStatus {
    let mut serial = SerialCom1::new();
    serial.init();
    serial.write_all(kernel_entry_message_line());
    serial.write_all(entry_done_message_line());
    EfiStatus::SUCCESS
}

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    let mut serial = SerialCom1::new();
    serial.init();
    serial.write_all(panic_message_line());
    loop {}
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::{
        entry_done_message_line, kernel_entry_message_line, panic_message_line, EfiStatus,
        BAUD_DIVISOR_38400, LINE_CONTROL_8N1, LINE_CONTROL_DLAB, LINE_STATUS_TRANSMITTER_EMPTY,
    };

    #[test]
    fn entry_message_line_matches_kernel_banner_with_crlf() {
        assert_eq!(
            kernel_entry_message_line(),
            b"tosm-os: kernel entry reached\r\n"
        );
    }

    #[test]
    fn panic_message_line_matches_kernel_canonical_panic_line() {
        assert_eq!(panic_message_line(), b"tosm-os: panic in uefi-entry\r\n");
    }

    #[test]
    fn entry_done_message_line_matches_kernel_canonical_completion_line() {
        assert_eq!(
            entry_done_message_line(),
            b"tosm-os: efi_main completed\r\n"
        );
    }

    #[test]
    fn efi_status_success_value_is_zero() {
        assert_eq!(EfiStatus::SUCCESS.0, 0);
    }

    #[test]
    fn line_status_transmitter_empty_bit_matches_uart_lsr_spec() {
        assert_eq!(LINE_STATUS_TRANSMITTER_EMPTY, 1 << 5);
    }

    #[test]
    fn uart_init_values_match_8n1_38400_profile() {
        assert_eq!(LINE_CONTROL_DLAB, 1 << 7);
        assert_eq!(LINE_CONTROL_8N1, 0b0000_0011);
        assert_eq!(BAUD_DIVISOR_38400, 3);
    }
}
