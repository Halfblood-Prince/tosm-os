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
const VGA_TEXT_BUFFER_PHYS_ADDR: usize = 0xB8000;
const VGA_TEXT_COLUMNS: usize = 80;
const VGA_TEXT_ROWS: usize = 25;
const VGA_COLOR_LIGHT_GRAY_ON_BLACK: u8 = 0x07;

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

/// Minimal VGA text buffer writer for early on-screen boot diagnostics.
struct VgaTextWriter {
    column: usize,
    row: usize,
    color: u8,
}

impl VgaTextWriter {
    const fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            color: VGA_COLOR_LIGHT_GRAY_ON_BLACK,
        }
    }

    fn write_all(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_byte(byte);
        }
    }

    fn init_for_boot_logs(&mut self) {
        self.clear_screen();
        self.column = 0;
        self.row = 0;
    }

    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.new_line();
            }
            b'\r' => {
                self.column = 0;
            }
            _ => {
                let index = vga_cell_index(self.row, self.column);
                write_vga_cell(index, byte, self.color);
                self.column += 1;
                if self.column == VGA_TEXT_COLUMNS {
                    self.new_line();
                }
            }
        }
    }

    fn new_line(&mut self) {
        self.column = 0;
        if self.row + 1 < VGA_TEXT_ROWS {
            self.row += 1;
        } else {
            self.scroll_up_one_row();
        }
        self.clear_row(self.row);
    }

    fn clear_screen(&mut self) {
        for row in 0..VGA_TEXT_ROWS {
            self.clear_row(row);
        }
    }

    fn clear_row(&mut self, row: usize) {
        for column in 0..VGA_TEXT_COLUMNS {
            let index = vga_cell_index(row, column);
            write_vga_cell(index, b' ', self.color);
        }
    }

    fn scroll_up_one_row(&mut self) {
        for row in 1..VGA_TEXT_ROWS {
            for column in 0..VGA_TEXT_COLUMNS {
                let from = vga_cell_index(row, column);
                let to = vga_cell_index(row - 1, column);
                let (ascii, color) = read_vga_cell(from);
                write_vga_cell(to, ascii, color);
            }
        }
    }
}

#[must_use]
const fn vga_cell_index(row: usize, column: usize) -> usize {
    row * VGA_TEXT_COLUMNS + column
}

fn write_vga_cell(index: usize, ascii: u8, color: u8) {
    let byte_offset = index.saturating_mul(2);
    let base_ptr = VGA_TEXT_BUFFER_PHYS_ADDR as *mut u8;
    // SAFETY: VGA text mode exposes a memory-mapped character buffer at physical address
    // 0xB8000; this writes one character byte + one color byte at a validated in-range offset.
    unsafe {
        core::ptr::write_volatile(base_ptr.add(byte_offset), ascii);
        core::ptr::write_volatile(base_ptr.add(byte_offset + 1), color);
    }
}

#[must_use]
fn read_vga_cell(index: usize) -> (u8, u8) {
    let byte_offset = index.saturating_mul(2);
    let base_ptr = VGA_TEXT_BUFFER_PHYS_ADDR as *const u8;
    // SAFETY: VGA text mode exposes a memory-mapped character buffer at physical address
    // 0xB8000; this reads one character byte + one color byte at a validated in-range offset.
    unsafe {
        let ascii = core::ptr::read_volatile(base_ptr.add(byte_offset));
        let color = core::ptr::read_volatile(base_ptr.add(byte_offset + 1));
        (ascii, color)
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
    let mut screen = VgaTextWriter::new();
    serial.init();
    screen.init_for_boot_logs();
    serial.write_all(kernel_entry_message_line());
    screen.write_all(kernel_entry_message_line());
    serial.write_all(entry_done_message_line());
    screen.write_all(entry_done_message_line());
    EfiStatus::SUCCESS
}

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    let mut serial = SerialCom1::new();
    let mut screen = VgaTextWriter::new();
    serial.init();
    screen.init_for_boot_logs();
    serial.write_all(panic_message_line());
    screen.write_all(panic_message_line());
    loop {}
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use core::array;

    use super::{
        entry_done_message_line, kernel_entry_message_line, panic_message_line, vga_cell_index,
        EfiStatus, BAUD_DIVISOR_38400, LINE_CONTROL_8N1, LINE_CONTROL_DLAB,
        LINE_STATUS_TRANSMITTER_EMPTY, VGA_TEXT_COLUMNS, VGA_TEXT_ROWS,
    };

    struct VgaWriterModel {
        column: usize,
        row: usize,
        cells: [[u8; VGA_TEXT_COLUMNS]; VGA_TEXT_ROWS],
    }

    impl VgaWriterModel {
        const BLANK: u8 = b' ';

        fn new() -> Self {
            Self {
                column: 0,
                row: 0,
                cells: array::from_fn(|_| [Self::BLANK; VGA_TEXT_COLUMNS]),
            }
        }

        fn init_for_boot_logs(&mut self) {
            self.clear_screen();
            self.column = 0;
            self.row = 0;
        }

        fn write_all(&mut self, bytes: &[u8]) {
            for &byte in bytes {
                self.write_byte(byte);
            }
        }

        fn write_byte(&mut self, byte: u8) {
            match byte {
                b'\n' => self.new_line(),
                b'\r' => self.column = 0,
                _ => {
                    self.cells[self.row][self.column] = byte;
                    self.column += 1;
                    if self.column == VGA_TEXT_COLUMNS {
                        self.new_line();
                    }
                }
            }
        }

        fn new_line(&mut self) {
            self.column = 0;
            if self.row + 1 < VGA_TEXT_ROWS {
                self.row += 1;
            } else {
                self.scroll_up_one_row();
            }
            self.clear_row(self.row);
        }

        fn clear_screen(&mut self) {
            for row in 0..VGA_TEXT_ROWS {
                self.clear_row(row);
            }
        }

        fn clear_row(&mut self, row: usize) {
            self.cells[row].fill(Self::BLANK);
        }

        fn scroll_up_one_row(&mut self) {
            for row in 1..VGA_TEXT_ROWS {
                self.cells[row - 1] = self.cells[row];
            }
        }

        fn row_bytes(&self, row: usize) -> [u8; VGA_TEXT_COLUMNS] {
            self.cells[row]
        }

        fn row_text_prefix(&self, row: usize) -> &[u8] {
            let row_bytes = &self.cells[row];
            let end = row_bytes
                .iter()
                .position(|byte| *byte == Self::BLANK)
                .unwrap_or(VGA_TEXT_COLUMNS);
            &row_bytes[..end]
        }
    }

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

    #[test]
    fn vga_cell_index_uses_row_major_layout() {
        assert_eq!(vga_cell_index(0, 0), 0);
        assert_eq!(vga_cell_index(0, 1), 1);
        assert_eq!(vga_cell_index(1, 0), VGA_TEXT_COLUMNS);
    }

    #[test]
    fn line_messages_fit_without_row_wrap() {
        let banner_columns = kernel_entry_message_line().len();
        let panic_columns = panic_message_line().len();
        let done_columns = entry_done_message_line().len();
        assert!(banner_columns < VGA_TEXT_COLUMNS);
        assert!(panic_columns < VGA_TEXT_COLUMNS);
        assert!(done_columns < VGA_TEXT_COLUMNS);
    }

    #[test]
    fn model_panic_line_renders_on_first_row_after_init() {
        let mut model = VgaWriterModel::new();
        model.write_all(b"dirty line\n");

        model.init_for_boot_logs();
        model.write_all(panic_message_line());

        let mut expected = [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS];
        expected[..("tosm-os: panic in uefi-entry".len())]
            .copy_from_slice(b"tosm-os: panic in uefi-entry");
        assert_eq!(model.row, 1);
        assert_eq!(model.column, 0);
        assert_eq!(model.row_bytes(0), expected);
        assert_eq!(
            model.row_bytes(1),
            [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]
        );
    }

    #[test]
    fn model_boot_transcript_renders_banner_then_done_on_distinct_rows() {
        let mut model = VgaWriterModel::new();
        model.init_for_boot_logs();

        model.write_all(kernel_entry_message_line());
        model.write_all(entry_done_message_line());

        assert_eq!(model.row, 2);
        assert_eq!(model.column, 0);
        assert_eq!(model.row_text_prefix(0), b"tosm-os: kernel entry reached");
        assert_eq!(model.row_text_prefix(1), b"tosm-os: efi_main completed");
        assert_eq!(
            model.row_bytes(2),
            [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]
        );
    }

    #[test]
    fn model_panic_transcript_reinitializes_screen_and_removes_old_boot_lines() {
        let mut model = VgaWriterModel::new();
        model.init_for_boot_logs();
        model.write_all(kernel_entry_message_line());
        model.write_all(entry_done_message_line());

        model.init_for_boot_logs();
        model.write_all(panic_message_line());

        assert_eq!(model.row, 1);
        assert_eq!(model.column, 0);
        assert_eq!(model.row_text_prefix(0), b"tosm-os: panic in uefi-entry");
        assert_ne!(
            model.row_text_prefix(0),
            b"tosm-os: kernel entry reached"
        );
        assert_ne!(model.row_text_prefix(0), b"tosm-os: efi_main completed");
        assert_eq!(
            model.row_bytes(1),
            [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]
        );
    }

    #[test]
    fn model_init_clears_screen_and_resets_cursor() {
        let mut model = VgaWriterModel::new();
        model.write_all(b"dirty");
        model.row = VGA_TEXT_ROWS - 1;
        model.column = VGA_TEXT_COLUMNS - 1;

        model.init_for_boot_logs();

        assert_eq!(model.row, 0);
        assert_eq!(model.column, 0);
        for row in 0..VGA_TEXT_ROWS {
            assert_eq!(
                model.row_bytes(row),
                [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]
            );
        }
    }

    #[test]
    fn model_newline_clears_destination_row() {
        let mut model = VgaWriterModel::new();
        model.init_for_boot_logs();
        model.write_all(b"AB\n");

        assert_eq!(model.row, 1);
        assert_eq!(model.column, 0);
        assert_eq!(model.row_bytes(0)[0..2], [b'A', b'B']);
        assert_eq!(
            model.row_bytes(1),
            [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]
        );
    }

    #[test]
    fn model_scroll_moves_rows_up_and_clears_last_row() {
        let mut model = VgaWriterModel::new();
        model.init_for_boot_logs();
        model.write_all(b"A\nB\n");
        model.row = VGA_TEXT_ROWS - 1;
        model.column = 0;
        model.cells[VGA_TEXT_ROWS - 2][0] = b'X';
        model.cells[VGA_TEXT_ROWS - 1][0] = b'Y';

        model.write_all(b"Z\n");

        assert_eq!(model.row, VGA_TEXT_ROWS - 1);
        assert_eq!(model.column, 0);
        assert_eq!(model.row_bytes(VGA_TEXT_ROWS - 2)[0], b'Z');
        assert_eq!(
            model.row_bytes(VGA_TEXT_ROWS - 1),
            [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]
        );
    }
}
