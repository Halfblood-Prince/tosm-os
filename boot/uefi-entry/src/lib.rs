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

/// Returns the deterministic interrupt-init line expected once early IDT setup is wired.
#[must_use]
pub const fn interrupt_init_message_line() -> &'static [u8] {
    kernel::boot_interrupt_init_line_bytes()
}

/// Returns the deterministic completion line expected before firmware returns success.
#[must_use]
pub const fn entry_done_message_line() -> &'static [u8] {
    kernel::boot_entry_done_line_bytes()
}

/// Returns the deterministic physical-memory init line expected after map modeling.
#[must_use]
pub const fn memory_init_message_line() -> &'static [u8] {
    kernel::boot_memory_init_line_bytes()
}

/// Returns the deterministic paging-plan line expected after frame-window selection.
#[must_use]
pub const fn paging_plan_message_line() -> &'static [u8] {
    kernel::boot_paging_plan_line_bytes()
}

/// Returns the deterministic paging-install line expected after table materialization.
#[must_use]
pub const fn paging_install_message_line() -> &'static [u8] {
    kernel::boot_paging_install_line_bytes()
}

/// Returns the deterministic heap-bootstrap line expected after frame-backed heap reservation.
#[must_use]
pub const fn heap_bootstrap_message_line() -> &'static [u8] {
    kernel::boot_heap_bootstrap_line_bytes()
}

/// Returns the deterministic heap-operation-cycle line after boot-time alloc/dealloc exercise.
#[must_use]
pub const fn heap_alloc_cycle_message_line() -> &'static [u8] {
    kernel::boot_heap_alloc_cycle_line_bytes()
}

/// Returns the deterministic exception line expected from early dispatch logging.
#[must_use]
pub fn exception_message_line(vector: u8) -> &'static [u8] {
    kernel::exception_log_line_bytes(vector)
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

    let _interrupt_report = kernel::init_early_interrupts();
    serial.write_all(interrupt_init_message_line());
    screen.write_all(interrupt_init_message_line());

    let simulated_dispatch = kernel::dispatch_exception(14);
    serial.write_all(simulated_dispatch.line.as_bytes());
    screen.write_all(simulated_dispatch.line.as_bytes());

    let memory_report = kernel::init_early_physical_memory();
    serial.write_all(memory_init_message_line());
    screen.write_all(memory_init_message_line());

    let paging_plan = kernel::init_early_paging_plan(memory_report);
    serial.write_all(paging_plan_message_line());
    screen.write_all(paging_plan_message_line());

    let paging_install_report = kernel::install_early_paging(paging_plan);
    serial.write_all(paging_install_message_line());
    screen.write_all(paging_install_message_line());

    let mut frame_allocator =
        kernel::EarlyFrameAllocator::from_install_report(paging_install_report);
    if let Ok(heap_bootstrap) =
        kernel::bootstrap_early_kernel_heap(&mut frame_allocator, paging_install_report)
    {
        serial.write_all(heap_bootstrap_message_line());
        screen.write_all(heap_bootstrap_message_line());

        let mut heap_allocator = kernel::EarlyHeapAllocator::from_bootstrap(heap_bootstrap);
        if kernel::run_early_heap_alloc_cycle(&mut heap_allocator).is_ok() {
            serial.write_all(heap_alloc_cycle_message_line());
            screen.write_all(heap_alloc_cycle_message_line());
        }
    }

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
        entry_done_message_line, exception_message_line, heap_alloc_cycle_message_line,
        heap_bootstrap_message_line, interrupt_init_message_line, kernel_entry_message_line,
        memory_init_message_line, paging_install_message_line, paging_plan_message_line,
        panic_message_line, vga_cell_index, EfiStatus, BAUD_DIVISOR_38400, LINE_CONTROL_8N1,
        LINE_CONTROL_DLAB, LINE_STATUS_TRANSMITTER_EMPTY, VGA_TEXT_COLUMNS, VGA_TEXT_ROWS,
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

        fn row_text_without_trailing_blanks(&self, row: usize) -> &[u8] {
            let row_bytes = &self.cells[row];
            let end = row_bytes
                .iter()
                .rposition(|byte| *byte != Self::BLANK)
                .map(|index| index + 1)
                .unwrap_or(0);
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
    fn interrupt_init_message_line_matches_kernel_canonical_interrupt_line() {
        assert_eq!(
            interrupt_init_message_line(),
            b"tosm-os: idt skeleton initialized\r\n"
        );
    }

    #[test]
    fn entry_done_message_line_matches_kernel_canonical_completion_line() {
        assert_eq!(
            entry_done_message_line(),
            b"tosm-os: efi_main completed\r\n"
        );
    }

    #[test]
    fn memory_init_message_line_matches_kernel_canonical_memory_line() {
        assert_eq!(
            memory_init_message_line(),
            b"tosm-os: memory init usable=0x3f790000 reserved=0x00811000 regions=5\r\n"
        );
    }

    #[test]
    fn paging_plan_message_line_matches_kernel_canonical_paging_plan_line() {
        assert_eq!(
            paging_plan_message_line(),
            b"tosm-os: paging plan frames=4 window=0x3f7ed000-0x3f7f1000 map4k=512\r\n"
        );
    }

    #[test]
    fn paging_install_message_line_matches_kernel_canonical_paging_install_line() {
        assert_eq!(
            paging_install_message_line(),
            b"tosm-os: paging install root=0x3f7ed000 span=0x40000000 entries=514\r\n"
        );
    }

    #[test]
    fn heap_bootstrap_message_line_matches_kernel_canonical_heap_bootstrap_line() {
        assert_eq!(
            heap_bootstrap_message_line(),
            b"tosm-os: heap bootstrap start=0x00400000 size=0x00004000 frames=4\r\n"
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
        let interrupt_columns = interrupt_init_message_line().len();
        let memory_columns = memory_init_message_line().len();
        let paging_plan_columns = paging_plan_message_line().len();
        let paging_install_columns = paging_install_message_line().len();
        let heap_columns = heap_bootstrap_message_line().len();
        let heap_cycle_columns = heap_alloc_cycle_message_line().len();
        let done_columns = entry_done_message_line().len();
        assert!(banner_columns < VGA_TEXT_COLUMNS);
        assert!(panic_columns < VGA_TEXT_COLUMNS);
        assert!(interrupt_columns < VGA_TEXT_COLUMNS);
        assert!(memory_columns < VGA_TEXT_COLUMNS);
        assert!(paging_plan_columns < VGA_TEXT_COLUMNS);
        assert!(paging_install_columns < VGA_TEXT_COLUMNS);
        assert!(heap_columns < VGA_TEXT_COLUMNS);
        assert!(heap_cycle_columns < VGA_TEXT_COLUMNS);
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
    fn model_boot_transcript_renders_banner_then_interrupt_then_done_on_distinct_rows() {
        let mut model = VgaWriterModel::new();
        model.init_for_boot_logs();

        model.write_all(kernel_entry_message_line());
        model.write_all(interrupt_init_message_line());
        model.write_all(exception_message_line(14));
        model.write_all(memory_init_message_line());
        model.write_all(paging_plan_message_line());
        model.write_all(paging_install_message_line());
        model.write_all(heap_bootstrap_message_line());
        model.write_all(heap_alloc_cycle_message_line());
        model.write_all(entry_done_message_line());

        assert_eq!(model.row, 9);
        assert_eq!(model.column, 0);
        assert_eq!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: kernel entry reached"
        );
        assert_eq!(
            model.row_text_without_trailing_blanks(1),
            b"tosm-os: idt skeleton initialized"
        );
        assert_eq!(
            model.row_text_without_trailing_blanks(2),
            b"tosm-os: exception vector 14 page fault"
        );
        assert_eq!(
            model.row_text_without_trailing_blanks(3),
            b"tosm-os: memory init usable=0x3f790000 reserved=0x00811000 regions=5"
        );
        assert_eq!(
            model.row_text_without_trailing_blanks(4),
            b"tosm-os: paging plan frames=4 window=0x3f7ed000-0x3f7f1000 map4k=512"
        );
        assert_eq!(
            model.row_text_without_trailing_blanks(5),
            b"tosm-os: paging install root=0x3f7ed000 span=0x40000000 entries=514"
        );
        assert_eq!(
            model.row_text_without_trailing_blanks(6),
            b"tosm-os: heap bootstrap start=0x00400000 size=0x00004000 frames=4"
        );
        assert_eq!(
            model.row_text_without_trailing_blanks(7),
            b"tosm-os: heap alloc cycle allocs=2 frees=2 cursor=0x00400000"
        );
        assert_eq!(
            model.row_text_without_trailing_blanks(8),
            b"tosm-os: efi_main completed"
        );
        assert_eq!(
            model.row_bytes(9),
            [VgaWriterModel::BLANK; VGA_TEXT_COLUMNS]
        );
    }

    #[test]
    fn model_panic_transcript_reinitializes_screen_and_removes_old_boot_lines() {
        let mut model = VgaWriterModel::new();
        model.init_for_boot_logs();
        model.write_all(kernel_entry_message_line());
        model.write_all(interrupt_init_message_line());
        model.write_all(exception_message_line(14));
        model.write_all(memory_init_message_line());
        model.write_all(paging_plan_message_line());
        model.write_all(paging_install_message_line());
        model.write_all(heap_bootstrap_message_line());
        model.write_all(heap_alloc_cycle_message_line());
        model.write_all(entry_done_message_line());

        model.init_for_boot_logs();
        model.write_all(panic_message_line());

        assert_eq!(model.row, 1);
        assert_eq!(model.column, 0);
        assert_eq!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: panic in uefi-entry"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: kernel entry reached"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: idt skeleton initialized"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: exception vector 14 page fault"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: memory init usable=0x3f790000 reserved=0x00811000 regions=5"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: paging plan frames=4 window=0x3f7ed000-0x3f7f1000 map4k=512"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: paging install root=0x3f7ed000 span=0x40000000 entries=514"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: heap bootstrap start=0x00400000 size=0x00004000 frames=4"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: heap alloc cycle allocs=2 frees=2 cursor=0x00400000"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: efi_main completed"
        );
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
    fn model_carriage_return_resets_column_and_overwrites_in_place() {
        let mut model = VgaWriterModel::new();
        model.init_for_boot_logs();
        model.write_all(b"ABCD\rxy");

        assert_eq!(model.row, 0);
        assert_eq!(model.column, 2);
        assert_eq!(model.row_bytes(0)[0..4], [b'x', b'y', b'C', b'D']);
    }

    #[test]
    fn model_width_boundary_wrap_advances_to_next_row_and_clears_it() {
        let mut model = VgaWriterModel::new();
        model.init_for_boot_logs();

        let full_row = [b'A'; VGA_TEXT_COLUMNS];
        model.write_all(&full_row);

        assert_eq!(model.row, 1);
        assert_eq!(model.column, 0);
        assert_eq!(model.row_bytes(0), [b'A'; VGA_TEXT_COLUMNS]);
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
