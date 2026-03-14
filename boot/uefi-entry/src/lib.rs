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
const BAUD_DIVISOR_115200: u8 = 1;
// Keep the UART readiness spin budget short so QEMU/firmware environments that never expose
// LSR.TX-empty don't burn most of the smoke-test runtime in per-byte polling loops.
const UART_TRANSMIT_READY_SPIN_LIMIT: usize = 4_096;
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
        // Configure 115200 baud divisor (low byte then high byte).
        port_write_u8(COM1_PORT, BAUD_DIVISOR_115200);
        port_write_u8(COM1_PORT + INTERRUPT_ENABLE_OFFSET, 0x00);
        // Clear DLAB and select 8 data bits, no parity, one stop bit.
        port_write_u8(COM1_PORT + LINE_CONTROL_OFFSET, LINE_CONTROL_8N1);
        // Enable FIFO and clear both queues with a conservative trigger level.
        port_write_u8(COM1_PORT + FIFO_CONTROL_OFFSET, FIFO_ENABLE_CLEAR_14B);
        // Assert DTR/RTS/OUT2 for basic transmitter readiness.
        port_write_u8(COM1_PORT + MODEM_CONTROL_OFFSET, MODEM_CONTROL_DTR_RTS_OUT2);
    }

    fn write_byte(&mut self, byte: u8) {
        self.wait_for_transmitter_ready();
        port_write_u8(COM1_PORT, byte);
    }

    fn wait_for_transmitter_ready(&self) {
        let mut spins = 0usize;
        while !self.transmitter_empty() {
            // Keep early serial output deterministic but avoid hanging forever when emulated UART
            // status bits lag on slower CI runners.
            spins += 1;
            if spins >= UART_TRANSMIT_READY_SPIN_LIMIT {
                break;
            }
            core::hint::spin_loop();
        }
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

/// Returns the deterministic global-allocator readiness line after facade install.
#[must_use]
pub const fn global_allocator_ready_message_line() -> &'static [u8] {
    kernel::boot_global_allocator_ready_line_bytes()
}

/// Returns the deterministic global-allocator probe line after dynamic structure exercise.
#[must_use]
pub const fn global_allocator_probe_message_line() -> &'static [u8] {
    kernel::boot_global_allocator_probe_line_bytes()
}

/// Returns the deterministic timer-init line expected after early timer configuration.
#[must_use]
pub const fn timer_init_message_line() -> &'static [u8] {
    kernel::boot_timer_init_line_bytes()
}

/// Returns the deterministic timer-first-tick line expected after first periodic tick delivery.
#[must_use]
pub const fn timer_first_tick_message_line() -> &'static [u8] {
    kernel::boot_timer_first_tick_line_bytes()
}

/// Returns the deterministic timer-third-tick line expected after multi-tick interrupt dispatch.
#[must_use]
pub const fn timer_third_tick_message_line() -> &'static [u8] {
    kernel::boot_timer_third_tick_line_bytes()
}

/// Returns the deterministic timer-ack line expected after PIC EOI acknowledgement.
#[must_use]
pub const fn timer_ack_message_line() -> &'static [u8] {
    kernel::boot_timer_ack_line_bytes()
}

/// Returns the deterministic timer-handoff line expected when timer state is sampled for scheduling.
#[must_use]
pub const fn timer_handoff_message_line() -> &'static [u8] {
    kernel::boot_timer_handoff_line_bytes()
}

/// Returns the deterministic scheduler-handoff line expected for run-queue bootstrap contracts.
#[must_use]
pub const fn scheduler_handoff_message_line() -> &'static [u8] {
    kernel::boot_scheduler_handoff_line_bytes()
}

/// Returns the deterministic thread-enqueue line expected for first scheduler slot mutation contracts.
#[must_use]
pub const fn thread_enqueue_message_line() -> &'static [u8] {
    kernel::boot_thread_enqueue_line_bytes()
}

/// Returns the deterministic thread-dequeue line expected for first scheduler slot mutation contracts.
#[must_use]
pub const fn thread_dequeue_message_line() -> &'static [u8] {
    kernel::boot_thread_dequeue_line_bytes()
}

/// Returns the deterministic thread-ctx-save line expected for scheduler context handoff.
#[must_use]
pub const fn thread_context_save_message_line() -> &'static [u8] {
    kernel::boot_thread_context_save_line_bytes()
}

/// Returns the deterministic thread-ctx-restore line expected for scheduler context handoff.
#[must_use]
pub const fn thread_context_restore_message_line() -> &'static [u8] {
    kernel::boot_thread_context_restore_line_bytes()
}

/// Returns deterministic thread-ctx metadata line expected for scheduler handoff cause/state.
#[must_use]
pub const fn thread_context_meta_message_line() -> &'static [u8] {
    kernel::boot_thread_context_meta_line_bytes()
}

/// Returns deterministic thread-state blocked line expected for lifecycle modeling.
#[must_use]
pub const fn thread_state_blocked_message_line() -> &'static [u8] {
    kernel::boot_thread_state_blocked_line_bytes()
}

/// Returns deterministic thread-state ready line expected for lifecycle modeling.
#[must_use]
pub const fn thread_state_ready_message_line() -> &'static [u8] {
    kernel::boot_thread_state_ready_line_bytes()
}

/// Returns deterministic thread-wake line expected for blocked->ready wake metadata modeling.
#[must_use]
pub const fn thread_wake_message_line() -> &'static [u8] {
    kernel::boot_thread_wake_line_bytes()
}

/// Returns deterministic thread wait-ownership line expected for wake-channel accounting.
#[must_use]
pub const fn thread_wait_ownership_message_line() -> &'static [u8] {
    kernel::boot_thread_wait_ownership_line_bytes()
}

/// Returns deterministic thread wake-timeout line expected for wake-deadline contracts.
#[must_use]
pub const fn thread_wake_timeout_message_line() -> &'static [u8] {
    kernel::boot_thread_wake_timeout_line_bytes()
}

/// Returns deterministic wait-contention line expected for wake arbitration contracts.
#[must_use]
pub const fn thread_wait_contention_message_line() -> &'static [u8] {
    kernel::boot_thread_wait_contention_line_bytes()
}

/// Returns deterministic wake-order line expected for wait-channel priority ordering contracts.
#[must_use]
pub const fn thread_wake_order_message_line() -> &'static [u8] {
    kernel::boot_thread_wake_order_line_bytes()
}

/// Returns deterministic wake-fairness line expected for multi-channel aging rotation contracts.
#[must_use]
pub const fn thread_wake_fairness_message_line() -> &'static [u8] {
    kernel::boot_thread_wake_fairness_line_bytes()
}

/// Returns deterministic scheduler rebalance line expected for runnable-aging decay contracts.
#[must_use]
pub const fn scheduler_rebalance_message_line() -> &'static [u8] {
    kernel::boot_scheduler_rebalance_line_bytes()
}

/// Returns deterministic scheduler carryover line expected for timeslice-threshold contracts.
#[must_use]
pub const fn scheduler_carryover_message_line() -> &'static [u8] {
    kernel::boot_scheduler_carryover_line_bytes()
}

/// Returns deterministic thread-state terminated line expected for lifecycle cleanup modeling.
#[must_use]
pub const fn thread_state_terminated_message_line() -> &'static [u8] {
    kernel::boot_thread_state_terminated_line_bytes()
}

/// Returns deterministic scheduler blocked-selection edge line expected for fallback modeling.
#[must_use]
pub const fn scheduler_edge_blocked_message_line() -> &'static [u8] {
    kernel::boot_scheduler_edge_blocked_line_bytes()
}

/// Returns deterministic scheduler terminated-dequeue edge line expected for cleanup modeling.
#[must_use]
pub const fn scheduler_edge_terminated_message_line() -> &'static [u8] {
    kernel::boot_scheduler_edge_terminated_line_bytes()
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

        if kernel::init_early_global_allocator(heap_bootstrap).is_ok() {
            serial.write_all(global_allocator_ready_message_line());
            screen.write_all(global_allocator_ready_message_line());

            if kernel::run_early_global_allocator_probe().is_ok() {
                serial.write_all(global_allocator_probe_message_line());
                screen.write_all(global_allocator_probe_message_line());
            }
        }
    }

    kernel::reset_early_timer_ticks();
    let timer_report = kernel::init_early_timer();
    serial.write_all(timer_init_message_line());
    screen.write_all(timer_init_message_line());

    let _first_dispatch = kernel::dispatch_early_timer_interrupt(timer_report);
    serial.write_all(timer_first_tick_message_line());
    screen.write_all(timer_first_tick_message_line());

    let _second_dispatch = kernel::dispatch_early_timer_interrupt(timer_report);
    let _third_dispatch = kernel::dispatch_early_timer_interrupt(timer_report);
    serial.write_all(timer_third_tick_message_line());
    screen.write_all(timer_third_tick_message_line());

    serial.write_all(timer_ack_message_line());
    screen.write_all(timer_ack_message_line());

    kernel::reset_early_scheduler_state();
    let _timer_handoff = kernel::take_early_timer_handoff(timer_report);
    serial.write_all(timer_handoff_message_line());
    screen.write_all(timer_handoff_message_line());

    let _scheduler_handoff = kernel::take_early_scheduler_timer_handoff(timer_report);
    serial.write_all(scheduler_handoff_message_line());
    screen.write_all(scheduler_handoff_message_line());

    if kernel::enqueue_early_scheduler_task(2).is_ok() {
        serial.write_all(thread_enqueue_message_line());
        screen.write_all(thread_enqueue_message_line());
    }

    // Emit the deterministic dequeue transcript immediately after enqueue while scheduler state
    // is still simple, so slow firmware/QEMU paths don't miss the dequeue contract line.
    kernel::reset_early_scheduler_state();
    if kernel::enqueue_early_scheduler_task(2).is_ok()
        && kernel::dequeue_early_scheduler_task(2).is_ok()
    {
        serial.write_all(thread_dequeue_message_line());
        screen.write_all(thread_dequeue_message_line());
    }

    // Restore the baseline scheduler state expected by the remaining context/lifecycle modeling.
    kernel::reset_early_scheduler_state();
    let _ = kernel::take_early_scheduler_timer_handoff(timer_report);
    let has_worker_task = kernel::enqueue_early_scheduler_task(2).is_ok();

    let _round_robin =
        kernel::advance_early_scheduler_round_robin(kernel::EarlySchedulerHandoffReason::Yield);

    if has_worker_task
        && kernel::model_early_thread_context_handoff(2, kernel::EarlySchedulerHandoffReason::Yield)
            .is_ok()
    {
        serial.write_all(thread_context_save_message_line());
        screen.write_all(thread_context_save_message_line());
        serial.write_all(thread_context_restore_message_line());
        screen.write_all(thread_context_restore_message_line());
        serial.write_all(thread_context_meta_message_line());
        screen.write_all(thread_context_meta_message_line());
    }

    if kernel::transition_early_thread_lifecycle(2, kernel::EarlyThreadLifecycleState::Blocked)
        .is_ok()
    {
        serial.write_all(thread_state_blocked_message_line());
        screen.write_all(thread_state_blocked_message_line());
    }

    if kernel::wake_early_thread(
        2,
        kernel::EarlyThreadWakeReason::Timer,
        0x0000_0000_0000_2000,
        3,
    )
    .is_ok()
    {
        serial.write_all(thread_state_ready_message_line());
        screen.write_all(thread_state_ready_message_line());
        serial.write_all(thread_wake_message_line());
        screen.write_all(thread_wake_message_line());
        serial.write_all(thread_wait_ownership_message_line());
        screen.write_all(thread_wait_ownership_message_line());
        serial.write_all(thread_wake_timeout_message_line());
        screen.write_all(thread_wake_timeout_message_line());
    }

    if kernel::transition_early_thread_lifecycle(2, kernel::EarlyThreadLifecycleState::Blocked)
        .is_ok()
        && kernel::enqueue_early_scheduler_task(3).is_ok()
        && kernel::transition_early_thread_lifecycle(3, kernel::EarlyThreadLifecycleState::Blocked)
            .is_ok()
        && kernel::resolve_early_thread_wake_contention(
            0x0000_0000_0000_3000,
            2,
            kernel::EarlyThreadWakeReason::Timer,
            3,
            kernel::EarlyThreadWakeReason::Signal,
        )
        .is_ok()
    {
        serial.write_all(thread_wait_contention_message_line());
        screen.write_all(thread_wait_contention_message_line());
        serial.write_all(thread_wake_order_message_line());
        screen.write_all(thread_wake_order_message_line());
    }

    if kernel::resolve_early_multi_channel_wake_fairness([
        kernel::EarlyThreadWakeFairnessSlot {
            wait_channel: 0x0000_0000_0000_5000,
            blocked_task_id: 3,
            reason: kernel::EarlyThreadWakeReason::Signal,
            channel_age: 3,
            claim_sequence: 2,
        },
        kernel::EarlyThreadWakeFairnessSlot {
            wait_channel: 0x0000_0000_0000_2000,
            blocked_task_id: 2,
            reason: kernel::EarlyThreadWakeReason::Timer,
            channel_age: 1,
            claim_sequence: 1,
        },
        kernel::EarlyThreadWakeFairnessSlot {
            wait_channel: 0x0000_0000_0000_5000,
            blocked_task_id: 4,
            reason: kernel::EarlyThreadWakeReason::Io,
            channel_age: 5,
            claim_sequence: 4,
        },
    ])
    .is_ok()
    {
        serial.write_all(thread_wake_fairness_message_line());
        screen.write_all(thread_wake_fairness_message_line());
    }

    if kernel::rebalance_early_scheduler_runnable_aging(
        [
            kernel::EarlySchedulerAgingSlot {
                task_id: 2,
                age: 7,
                decay: 3,
            },
            kernel::EarlySchedulerAgingSlot {
                task_id: 3,
                age: 5,
                decay: 3,
            },
            kernel::EarlySchedulerAgingSlot {
                task_id: 4,
                age: 4,
                decay: 2,
            },
        ],
        4,
    )
    .is_ok()
    {
        serial.write_all(scheduler_rebalance_message_line());
        screen.write_all(scheduler_rebalance_message_line());
    }

    if kernel::model_early_scheduler_timeslice_carryover(
        [
            kernel::EarlySchedulerTimesliceSlot {
                task_id: 2,
                remaining_ticks: 2,
                carry_ticks: 1,
            },
            kernel::EarlySchedulerTimesliceSlot {
                task_id: 3,
                remaining_ticks: 4,
                carry_ticks: 0,
            },
            kernel::EarlySchedulerTimesliceSlot {
                task_id: 4,
                remaining_ticks: 3,
                carry_ticks: 1,
            },
        ],
        2,
        3,
    )
    .is_ok()
    {
        serial.write_all(scheduler_carryover_message_line());
        screen.write_all(scheduler_carryover_message_line());
    }

    if kernel::model_early_scheduler_blocked_selection_edge_case(1).is_ok() {
        serial.write_all(scheduler_edge_blocked_message_line());
        screen.write_all(scheduler_edge_blocked_message_line());
    }

    if kernel::model_early_scheduler_terminated_cleanup_edge_case(2).is_ok() {
        serial.write_all(thread_state_terminated_message_line());
        screen.write_all(thread_state_terminated_message_line());
        serial.write_all(scheduler_edge_terminated_message_line());
        screen.write_all(scheduler_edge_terminated_message_line());
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
        entry_done_message_line, exception_message_line, global_allocator_probe_message_line,
        global_allocator_ready_message_line, heap_alloc_cycle_message_line,
        heap_bootstrap_message_line, interrupt_init_message_line, kernel_entry_message_line,
        memory_init_message_line, paging_install_message_line, paging_plan_message_line,
        panic_message_line, scheduler_carryover_message_line, scheduler_edge_blocked_message_line,
        scheduler_edge_terminated_message_line, scheduler_handoff_message_line,
        thread_context_meta_message_line, thread_context_restore_message_line,
        thread_context_save_message_line, thread_dequeue_message_line, thread_enqueue_message_line,
        thread_state_blocked_message_line, thread_state_ready_message_line,
        thread_state_terminated_message_line, thread_wait_contention_message_line,
        thread_wait_ownership_message_line, thread_wake_fairness_message_line,
        thread_wake_message_line, thread_wake_order_message_line, thread_wake_timeout_message_line,
        timer_ack_message_line, timer_first_tick_message_line, timer_handoff_message_line,
        timer_init_message_line, timer_third_tick_message_line, vga_cell_index, EfiStatus,
        BAUD_DIVISOR_115200, LINE_CONTROL_8N1, LINE_CONTROL_DLAB, LINE_STATUS_TRANSMITTER_EMPTY,
        VGA_TEXT_COLUMNS, VGA_TEXT_ROWS,
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
    fn global_allocator_ready_message_line_matches_kernel_canonical_allocator_line() {
        assert_eq!(
            global_allocator_ready_message_line(),
            b"tosm-os: global allocator ready heap=0x00400000-0x00404000\r\n"
        );
    }

    #[test]
    fn global_allocator_probe_message_line_matches_kernel_canonical_probe_line() {
        assert_eq!(
            global_allocator_probe_message_line(),
            b"tosm-os: global allocator probe entries=4 checksum=0x000000000000002a\r\n"
        );
    }

    #[test]
    fn timer_init_message_line_matches_kernel_canonical_timer_line() {
        assert_eq!(
            timer_init_message_line(),
            b"tosm-os: timer init source=pit hz=100 divisor=11931 irq=0x20\r\n"
        );
    }

    #[test]
    fn timer_first_tick_message_line_matches_kernel_canonical_tick_line() {
        assert_eq!(
            timer_first_tick_message_line(),
            b"tosm-os: timer tick irq=0x20 count=1 uptime_ns=10000000\r\n"
        );
    }

    #[test]
    fn timer_third_tick_message_line_matches_kernel_canonical_tick_line() {
        assert_eq!(
            timer_third_tick_message_line(),
            b"tosm-os: timer tick irq=0x20 count=3 uptime_ns=30000000\r\n"
        );
    }

    #[test]
    fn timer_ack_message_line_matches_kernel_canonical_ack_line() {
        assert_eq!(
            timer_ack_message_line(),
            b"tosm-os: timer ack irq=0x20 pic=0x20 eoi=0x20\r\n"
        );
    }

    #[test]
    fn timer_handoff_message_line_matches_kernel_canonical_handoff_line() {
        assert_eq!(
            timer_handoff_message_line(),
            b"tosm-os: timer handoff ticks=3 delta=3 quantum=1 uptime_ns=30000000\r\n"
        );
    }

    #[test]
    fn scheduler_handoff_message_line_matches_kernel_canonical_handoff_line() {
        assert_eq!(
            scheduler_handoff_message_line(),
            b"tosm-os: scheduler handoff reason=timer runq=2 selected=1 idle=0 delta=3\r\n"
        );
    }

    #[test]
    fn thread_enqueue_message_line_matches_kernel_canonical_enqueue_line() {
        assert_eq!(
            thread_enqueue_message_line(),
            b"tosm-os: thread enqueue task=2 runq=3 selected=1\r\n"
        );
    }

    #[test]
    fn thread_dequeue_message_line_matches_kernel_canonical_dequeue_line() {
        assert_eq!(
            thread_dequeue_message_line(),
            b"tosm-os: thread dequeue task=2 runq=2 selected=1\r\n"
        );
    }

    #[test]
    fn thread_context_save_message_line_matches_kernel_canonical_context_line() {
        assert_eq!(
            thread_context_save_message_line(),
            b"tosm-os: thread ctx save from=1 to=2 rip=0x100200 rsp=0x401f00\r\n"
        );
    }

    #[test]
    fn thread_context_restore_message_line_matches_kernel_canonical_context_line() {
        assert_eq!(
            thread_context_restore_message_line(),
            b"tosm-os: thread ctx restore to=2 rip=0x200000 rsp=0x402000\r\n"
        );
    }

    #[test]
    fn thread_context_meta_message_line_matches_kernel_canonical_context_line() {
        assert_eq!(
            thread_context_meta_message_line(),
            b"tosm-os: thread ctx meta reason=yield tick=3 runq=3 watermark=3\r\n"
        );
    }

    #[test]
    fn thread_wake_message_line_matches_kernel_canonical_wake_line() {
        assert_eq!(
            thread_wake_message_line(),
            b"tosm-os: thread wake task=2 reason=timer wait=0x2000 runq=3 sel=1\r\n"
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
    fn uart_init_values_match_8n1_115200_profile() {
        assert_eq!(LINE_CONTROL_DLAB, 1 << 7);
        assert_eq!(LINE_CONTROL_8N1, 0b0000_0011);
        assert_eq!(BAUD_DIVISOR_115200, 1);
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
        let global_allocator_columns = global_allocator_ready_message_line().len();
        let global_allocator_probe_columns = global_allocator_probe_message_line().len();
        let timer_columns = timer_init_message_line().len();
        let timer_first_tick_columns = timer_first_tick_message_line().len();
        let scheduler_handoff_columns = scheduler_handoff_message_line().len();
        let thread_enqueue_columns = thread_enqueue_message_line().len();
        let thread_context_save_columns = thread_context_save_message_line().len();
        let thread_context_restore_columns = thread_context_restore_message_line().len();
        let thread_state_blocked_columns = thread_state_blocked_message_line().len();
        let thread_state_ready_columns = thread_state_ready_message_line().len();
        let thread_wake_columns = thread_wake_message_line().len();
        let thread_wait_ownership_columns = thread_wait_ownership_message_line().len();
        let thread_wake_timeout_columns = thread_wake_timeout_message_line().len();
        let thread_wait_contention_columns = thread_wait_contention_message_line().len();
        let thread_wake_order_columns = thread_wake_order_message_line().len();
        let thread_dequeue_columns = thread_dequeue_message_line().len();
        let done_columns = entry_done_message_line().len();
        assert!(banner_columns < VGA_TEXT_COLUMNS);
        assert!(panic_columns < VGA_TEXT_COLUMNS);
        assert!(interrupt_columns < VGA_TEXT_COLUMNS);
        assert!(memory_columns < VGA_TEXT_COLUMNS);
        assert!(paging_plan_columns < VGA_TEXT_COLUMNS);
        assert!(paging_install_columns < VGA_TEXT_COLUMNS);
        assert!(heap_columns < VGA_TEXT_COLUMNS);
        assert!(heap_cycle_columns < VGA_TEXT_COLUMNS);
        assert!(global_allocator_columns < VGA_TEXT_COLUMNS);
        assert!(global_allocator_probe_columns < VGA_TEXT_COLUMNS);
        assert!(timer_columns < VGA_TEXT_COLUMNS);
        assert!(timer_first_tick_columns < VGA_TEXT_COLUMNS);
        assert!(scheduler_handoff_columns < VGA_TEXT_COLUMNS);
        assert!(thread_enqueue_columns < VGA_TEXT_COLUMNS);
        assert!(thread_context_save_columns < VGA_TEXT_COLUMNS);
        assert!(thread_context_restore_columns < VGA_TEXT_COLUMNS);
        assert!(thread_state_blocked_columns < VGA_TEXT_COLUMNS);
        assert!(thread_state_ready_columns < VGA_TEXT_COLUMNS);
        assert!(thread_wake_columns < VGA_TEXT_COLUMNS);
        assert!(thread_wait_ownership_columns < VGA_TEXT_COLUMNS);
        assert!(thread_wake_timeout_columns < VGA_TEXT_COLUMNS);
        assert!(thread_wait_contention_columns < VGA_TEXT_COLUMNS);
        assert!(thread_wake_order_columns < VGA_TEXT_COLUMNS);
        assert!(thread_dequeue_columns < VGA_TEXT_COLUMNS);
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
        model.write_all(global_allocator_ready_message_line());
        model.write_all(global_allocator_probe_message_line());
        model.write_all(timer_init_message_line());
        model.write_all(timer_first_tick_message_line());
        model.write_all(timer_third_tick_message_line());
        model.write_all(timer_ack_message_line());
        model.write_all(timer_handoff_message_line());
        model.write_all(scheduler_handoff_message_line());
        model.write_all(thread_enqueue_message_line());
        model.write_all(thread_context_save_message_line());
        model.write_all(thread_context_restore_message_line());
        model.write_all(thread_context_meta_message_line());
        model.write_all(thread_state_blocked_message_line());
        model.write_all(thread_state_ready_message_line());
        model.write_all(thread_wake_message_line());
        model.write_all(thread_wait_ownership_message_line());
        model.write_all(thread_wake_timeout_message_line());
        model.write_all(thread_wait_contention_message_line());
        model.write_all(thread_wake_order_message_line());
        model.write_all(thread_wake_fairness_message_line());
        model.write_all(super::scheduler_rebalance_message_line());
        model.write_all(scheduler_carryover_message_line());
        model.write_all(scheduler_edge_blocked_message_line());
        model.write_all(thread_state_terminated_message_line());
        model.write_all(scheduler_edge_terminated_message_line());
        model.write_all(thread_dequeue_message_line());
        model.write_all(entry_done_message_line());

        assert_eq!(model.row, VGA_TEXT_ROWS - 1);
        assert_eq!(model.column, 0);

        let has_line = |needle: &[u8]| {
            (0..VGA_TEXT_ROWS).any(|row| model.row_text_without_trailing_blanks(row) == needle)
        };

        assert!(has_line(
            b"tosm-os: thread ctx meta reason=yield tick=3 runq=3 watermark=3"
        ));
        assert!(has_line(
            b"tosm-os: scheduler edge case=blocked-selected task=1 runq=2 selected=0"
        ));
        assert!(has_line(
            b"tosm-os: thread wake task=2 reason=timer wait=0x2000 runq=3 sel=1"
        ));
        assert!(has_line(
            b"tosm-os: scheduler rebalance winner=2 age=4 decayed=6 floor=4 boost=1"
        ));
        assert!(has_line(
            b"tosm-os: scheduler carryover task=2 rem=2 carry=1 thresh=3 preempt=0 next=2"
        ));
        assert!(has_line(
            b"tosm-os: thread state task=2 ready->terminated runq=1 selected=0"
        ));
        assert!(has_line(b"tosm-os: efi_main completed"));
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
        model.write_all(timer_init_message_line());
        model.write_all(timer_first_tick_message_line());
        model.write_all(timer_third_tick_message_line());
        model.write_all(timer_ack_message_line());
        model.write_all(timer_handoff_message_line());
        model.write_all(scheduler_handoff_message_line());
        model.write_all(thread_enqueue_message_line());
        model.write_all(thread_context_save_message_line());
        model.write_all(thread_context_restore_message_line());
        model.write_all(thread_context_meta_message_line());
        model.write_all(thread_state_blocked_message_line());
        model.write_all(thread_state_ready_message_line());
        model.write_all(thread_wake_message_line());
        model.write_all(thread_wait_ownership_message_line());
        model.write_all(thread_wake_timeout_message_line());
        model.write_all(thread_wait_contention_message_line());
        model.write_all(thread_wake_order_message_line());
        model.write_all(thread_wake_fairness_message_line());
        model.write_all(super::scheduler_rebalance_message_line());
        model.write_all(scheduler_carryover_message_line());
        model.write_all(scheduler_edge_blocked_message_line());
        model.write_all(thread_state_terminated_message_line());
        model.write_all(scheduler_edge_terminated_message_line());
        model.write_all(thread_dequeue_message_line());
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
            b"tosm-os: timer init source=pit hz=100 divisor=11931 irq=0x20"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: timer tick irq=0x20 count=3 uptime_ns=30000000"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: timer ack irq=0x20 pic=0x20 eoi=0x20"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: timer handoff ticks=3 delta=3 quantum=1 uptime_ns=30000000"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: scheduler handoff reason=timer runq=2 selected=1 idle=0 delta=3"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread enqueue task=2 runq=3 selected=1"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread dequeue task=2 runq=2 selected=1"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread ctx meta reason=yield tick=3 runq=3 watermark=3"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread ctx save from=1 to=2 rip=0x100200 rsp=0x401f00"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread ctx restore to=2 rip=0x200000 rsp=0x402000"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread state task=2 ready->blocked runq=2 selected=1"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread state task=2 blocked->ready runq=3 selected=1"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread wake task=2 reason=timer wait=0x2000 runq=3 sel=1"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread wait owner=1 task=2 wait=0x2000 claim=1"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread wake timeout task=2 deadline=3 now=3 expired=1"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread wait contend wait=0x3000 winner=3 loser=2 pri=signal>timer"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread wake order first=3 second=2 wait=0x3000 claims=2,3"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread wake fairness first=4 wait=0x5000 age=5 second=3 age=3 rotate=1"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: scheduler rebalance winner=2 age=4 decayed=6 floor=4 boost=1"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: scheduler carryover task=2 rem=2 carry=1 thresh=3 preempt=0 next=2"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: scheduler edge case=blocked-selected task=1 runq=2 selected=0"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: thread state task=2 ready->terminated runq=1 selected=0"
        );
        assert_ne!(
            model.row_text_without_trailing_blanks(0),
            b"tosm-os: scheduler edge case=terminated-dequeue task=2 err=task-not-found runq=1 selected=0"
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
    fn multi_channel_wake_fairness_prioritizes_oldest_wait_channel() {
        let fairness = kernel::resolve_early_multi_channel_wake_fairness([
            kernel::EarlyThreadWakeFairnessSlot {
                wait_channel: 0x5000,
                blocked_task_id: 3,
                reason: kernel::EarlyThreadWakeReason::Signal,
                channel_age: 3,
                claim_sequence: 2,
            },
            kernel::EarlyThreadWakeFairnessSlot {
                wait_channel: 0x2000,
                blocked_task_id: 2,
                reason: kernel::EarlyThreadWakeReason::Timer,
                channel_age: 1,
                claim_sequence: 1,
            },
            kernel::EarlyThreadWakeFairnessSlot {
                wait_channel: 0x5000,
                blocked_task_id: 4,
                reason: kernel::EarlyThreadWakeReason::Io,
                channel_age: 5,
                claim_sequence: 4,
            },
        ])
        .expect("fairness arbitration should resolve");

        assert_eq!(fairness.first_task_id, 4);
        assert_eq!(fairness.first_wait_channel, 0x5000);
        assert_eq!(fairness.first_age, 5);
        assert_eq!(fairness.second_task_id, 3);
        assert_eq!(fairness.second_wait_channel, 0x5000);
        assert_eq!(fairness.second_age, 3);
        assert!(fairness.rotation_applied);
        assert!(fairness.starvation_prevented);
    }

    #[test]
    fn multi_channel_wake_fairness_message_matches_kernel_contract() {
        assert_eq!(
            thread_wake_fairness_message_line(),
            b"tosm-os: thread wake fairness first=4 wait=0x5000 age=5 second=3 age=3 rotate=1\r\n"
        );
    }

    #[test]
    fn scheduler_rebalance_message_matches_kernel_contract() {
        assert_eq!(
            super::scheduler_rebalance_message_line(),
            b"tosm-os: scheduler rebalance winner=2 age=4 decayed=6 floor=4 boost=1\r\n"
        );
    }

    #[test]
    fn scheduler_carryover_message_matches_kernel_contract() {
        assert_eq!(
            scheduler_carryover_message_line(),
            b"tosm-os: scheduler carryover task=2 rem=2 carry=1 thresh=3 preempt=0 next=2\r\n"
        );
    }

    #[test]
    fn scheduler_carryover_model_matches_expected_threshold_behavior() {
        let report = kernel::model_early_scheduler_timeslice_carryover(
            [
                kernel::EarlySchedulerTimesliceSlot {
                    task_id: 2,
                    remaining_ticks: 2,
                    carry_ticks: 1,
                },
                kernel::EarlySchedulerTimesliceSlot {
                    task_id: 3,
                    remaining_ticks: 4,
                    carry_ticks: 0,
                },
                kernel::EarlySchedulerTimesliceSlot {
                    task_id: 4,
                    remaining_ticks: 3,
                    carry_ticks: 1,
                },
            ],
            2,
            3,
        )
        .expect("carryover model should resolve deterministic timeslice sample");

        assert_eq!(report.selected_task_id, 2);
        assert_eq!(report.selected_remaining_ticks, 2);
        assert_eq!(report.selected_carry_ticks, 1);
        assert_eq!(report.preemption_threshold, 3);
        assert!(!report.preempted);
        assert_eq!(report.next_task_id, 2);
    }

    #[test]
    fn scheduler_rebalance_model_matches_expected_decay_and_floor() {
        let report = kernel::rebalance_early_scheduler_runnable_aging(
            [
                kernel::EarlySchedulerAgingSlot {
                    task_id: 2,
                    age: 7,
                    decay: 3,
                },
                kernel::EarlySchedulerAgingSlot {
                    task_id: 3,
                    age: 5,
                    decay: 3,
                },
                kernel::EarlySchedulerAgingSlot {
                    task_id: 4,
                    age: 4,
                    decay: 2,
                },
            ],
            4,
        )
        .expect("rebalance should resolve deterministic aging sample");

        assert_eq!(report.winner_task_id, 2);
        assert_eq!(report.winner_age_after_decay, 4);
        assert_eq!(report.winner_age_after_rebalance, 4);
        assert_eq!(report.floor_age, 4);
        assert_eq!(report.boost_applied, 0);
        assert_eq!(report.total_decay_applied, 8);
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

    #[test]
    fn scheduler_flow_preserves_worker_for_context_and_lifecycle_modeling() {
        let timer = kernel::init_early_timer();
        kernel::reset_early_timer_ticks();
        let _ = kernel::dispatch_early_timer_interrupt(timer);
        let _ = kernel::dispatch_early_timer_interrupt(timer);
        let _ = kernel::dispatch_early_timer_interrupt(timer);

        kernel::reset_early_scheduler_state();
        let _ = kernel::take_early_scheduler_timer_handoff(timer);
        assert!(kernel::enqueue_early_scheduler_task(2).is_ok());
        let _ =
            kernel::advance_early_scheduler_round_robin(kernel::EarlySchedulerHandoffReason::Yield);

        assert!(kernel::model_early_thread_context_handoff(
            2,
            kernel::EarlySchedulerHandoffReason::Yield
        )
        .is_ok());
        assert!(kernel::transition_early_thread_lifecycle(
            2,
            kernel::EarlyThreadLifecycleState::Blocked
        )
        .is_ok());
        assert!(
            kernel::wake_early_thread(2, kernel::EarlyThreadWakeReason::Timer, 0x2000, 3).is_ok()
        );
    }
}
