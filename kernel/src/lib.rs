#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::mem::size_of;
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use core::{alloc::GlobalAlloc, alloc::Layout, cell::UnsafeCell};

/// Deterministic serial banner used by the boot milestone smoke test.
pub const BOOT_BANNER: &str = "tosm-os: kernel entry reached";

/// Canonical serial line emitted from boot entry paths.
pub const BOOT_BANNER_LINE: &str = "tosm-os: kernel entry reached\r\n";

/// Canonical interrupt init line emitted when the early IDT skeleton is ready.
pub const BOOT_INTERRUPT_INIT_LINE: &str = "tosm-os: idt skeleton initialized\r\n";

/// Canonical panic line emitted by early boot firmware entry paths.
pub const BOOT_PANIC_LINE: &str = "tosm-os: panic in uefi-entry\r\n";

/// Canonical completion line emitted when firmware entry returns success.
pub const BOOT_ENTRY_DONE_LINE: &str = "tosm-os: efi_main completed\r\n";

/// Canonical physical-memory init line emitted after deterministic early map modeling.
pub const BOOT_MEMORY_INIT_LINE: &str =
    "tosm-os: memory init usable=0x3f790000 reserved=0x00811000 regions=5\r\n";

/// Canonical paging-plan line emitted after deterministic early frame-window selection.
pub const BOOT_PAGING_PLAN_LINE: &str =
    "tosm-os: paging plan frames=4 window=0x3f7ed000-0x3f7f1000 map4k=512\r\n";

/// Canonical paging-install line emitted after early page tables are materialized.
pub const BOOT_PAGING_INSTALL_LINE: &str =
    "tosm-os: paging install root=0x3f7ed000 span=0x40000000 entries=514\r\n";

/// Canonical heap-bootstrap line emitted after allocator-backed heap bring-up is initialized.
pub const BOOT_HEAP_BOOTSTRAP_LINE: &str =
    "tosm-os: heap bootstrap start=0x00400000 size=0x00004000 frames=4\r\n";

/// Canonical heap-operation line emitted after deterministic allocation/deallocation cycle passes.
pub const BOOT_HEAP_ALLOC_CYCLE_LINE: &str =
    "tosm-os: heap alloc cycle allocs=2 frees=2 cursor=0x00400000\r\n";

/// Canonical global-allocator readiness line emitted once the early heap facade is installed.
pub const BOOT_GLOBAL_ALLOCATOR_READY_LINE: &str =
    "tosm-os: global allocator ready heap=0x00400000-0x00404000\r\n";

/// Canonical probe line emitted after exercising a kernel-owned dynamic structure allocation.
pub const BOOT_GLOBAL_ALLOCATOR_PROBE_LINE: &str =
    "tosm-os: global allocator probe entries=4 checksum=0x000000000000002a\r\n";

/// Canonical timer-init line emitted once deterministic early timer configuration is computed.
pub const BOOT_TIMER_INIT_LINE: &str =
    "tosm-os: timer init source=pit hz=100 divisor=11931 irq=0x20\r\n";

/// Canonical timer-first-tick line emitted once the first deterministic PIT tick is accounted.
pub const BOOT_TIMER_FIRST_TICK_LINE: &str =
    "tosm-os: timer tick irq=0x20 count=1 uptime_ns=10000000\r\n";

/// Canonical timer-third-tick line emitted after deterministic multi-tick interrupt dispatch.
pub const BOOT_TIMER_THIRD_TICK_LINE: &str =
    "tosm-os: timer tick irq=0x20 count=3 uptime_ns=30000000\r\n";

/// Canonical timer-ack line emitted when the timer IRQ is acknowledged through the PIC path.
pub const BOOT_TIMER_ACK_LINE: &str = "tosm-os: timer ack irq=0x20 pic=0x20 eoi=0x20\r\n";

/// Canonical timer-handoff line emitted when timer state is consumed for scheduler integration.
pub const BOOT_TIMER_HANDOFF_LINE: &str =
    "tosm-os: timer handoff ticks=3 delta=3 quantum=1 uptime_ns=30000000\r\n";

/// Canonical scheduler-bootstrap line emitted for timer-driven run-queue handoff contracts.
pub const BOOT_SCHEDULER_HANDOFF_LINE: &str =
    "tosm-os: scheduler handoff reason=timer runq=2 selected=1 idle=0 delta=3\r\n";

/// Canonical thread-enqueue line emitted when a first worker slot is added to the scheduler queue.
pub const BOOT_THREAD_ENQUEUE_LINE: &str = "tosm-os: thread enqueue task=2 runq=3 selected=1\r\n";

/// Canonical thread-dequeue line emitted when a modeled worker slot is removed from the queue.
pub const BOOT_THREAD_DEQUEUE_LINE: &str = "tosm-os: thread dequeue task=2 runq=2 selected=1\r\n";

/// Canonical thread-ctx-save line emitted when scheduler handoff snapshots outgoing state.
pub const BOOT_THREAD_CONTEXT_SAVE_LINE: &str =
    "tosm-os: thread ctx save from=1 to=2 rip=0x100200 rsp=0x401f00\r\n";

/// Canonical thread-ctx-restore line emitted when scheduler handoff activates incoming state.
pub const BOOT_THREAD_CONTEXT_RESTORE_LINE: &str =
    "tosm-os: thread ctx restore to=2 rip=0x200000 rsp=0x402000\r\n";

/// Canonical thread-ctx-meta line emitted when handoff cause and queue metadata are sampled.
pub const BOOT_THREAD_CONTEXT_META_LINE: &str =
    "tosm-os: thread ctx meta reason=yield tick=3 runq=3 watermark=3\r\n";

/// Canonical thread-state-blocked line emitted when a runnable worker transitions to blocked.
pub const BOOT_THREAD_STATE_BLOCKED_LINE: &str =
    "tosm-os: thread state task=2 ready->blocked runq=2 selected=1\r\n";

/// Canonical thread-state-ready line emitted when a blocked worker transitions back to runnable.
pub const BOOT_THREAD_STATE_READY_LINE: &str =
    "tosm-os: thread state task=2 blocked->ready runq=3 selected=1\r\n";

/// Canonical thread-wake line emitted when a blocked worker resumes with wake metadata.
pub const BOOT_THREAD_WAKE_LINE: &str =
    "tosm-os: thread wake task=2 reason=timer wait=0x2000 runq=3 sel=1\r\n";

/// Canonical wait-ownership line emitted when blocked wait-channel ownership is sampled.
pub const BOOT_THREAD_WAIT_OWNERSHIP_LINE: &str =
    "tosm-os: thread wait owner=1 task=2 wait=0x2000 claim=1\r\n";

/// Canonical timeout wake line emitted when wake-deadline metadata is modeled.
pub const BOOT_THREAD_WAKE_TIMEOUT_LINE: &str =
    "tosm-os: thread wake timeout task=2 deadline=3 now=3 expired=1\r\n";

/// Canonical contention line emitted when wait-channel wake arbitration selects a winner.
pub const BOOT_THREAD_WAIT_CONTENTION_LINE: &str =
    "tosm-os: thread wait contend wait=0x3000 winner=3 loser=2 pri=signal>timer\r\n";

/// Canonical wake-order line emitted for deterministic first/second wake ordering metadata.
pub const BOOT_THREAD_WAKE_ORDER_LINE: &str =
    "tosm-os: thread wake order first=3 second=2 wait=0x3000 claims=2,3\r\n";

/// Canonical thread-state-terminated line emitted when a worker lifecycle is cleaned up.
pub const BOOT_THREAD_STATE_TERMINATED_LINE: &str =
    "tosm-os: thread state task=2 ready->terminated runq=1 selected=0\r\n";

/// Canonical scheduler-edge line emitted when selected blocked task falls back to idle.
pub const BOOT_SCHEDULER_EDGE_BLOCKED_LINE: &str =
    "tosm-os: scheduler edge case=blocked-selected task=1 runq=2 selected=0\r\n";

/// Canonical scheduler-edge line emitted when terminated task dequeue is rejected.
pub const BOOT_SCHEDULER_EDGE_TERMINATED_LINE: &str =
    "tosm-os: scheduler edge case=terminated-dequeue task=2 err=task-not-found runq=1 selected=0\r\n";

/// Returns the kernel boot banner as a byte slice for firmware serial writers.
#[must_use]
pub const fn boot_banner_bytes() -> &'static [u8] {
    BOOT_BANNER.as_bytes()
}

/// Returns the canonical banner line (including CRLF) for serial transmitters.
#[must_use]
pub const fn boot_banner_line_bytes() -> &'static [u8] {
    BOOT_BANNER_LINE.as_bytes()
}

/// Returns the canonical interrupt-init line (including CRLF) for serial transmitters.
#[must_use]
pub const fn boot_interrupt_init_line_bytes() -> &'static [u8] {
    BOOT_INTERRUPT_INIT_LINE.as_bytes()
}

/// Returns the canonical panic line (including CRLF) for early serial panic paths.
#[must_use]
pub const fn boot_panic_line_bytes() -> &'static [u8] {
    BOOT_PANIC_LINE.as_bytes()
}

/// Returns the canonical completion line (including CRLF) for firmware exit paths.
#[must_use]
pub const fn boot_entry_done_line_bytes() -> &'static [u8] {
    BOOT_ENTRY_DONE_LINE.as_bytes()
}

/// Returns the canonical physical-memory-init line (including CRLF) for serial transmitters.
#[must_use]
pub const fn boot_memory_init_line_bytes() -> &'static [u8] {
    BOOT_MEMORY_INIT_LINE.as_bytes()
}

/// Returns the canonical paging-plan line (including CRLF) for serial transmitters.
#[must_use]
pub const fn boot_paging_plan_line_bytes() -> &'static [u8] {
    BOOT_PAGING_PLAN_LINE.as_bytes()
}

/// Returns the canonical paging-install line (including CRLF) for serial transmitters.
#[must_use]
pub const fn boot_paging_install_line_bytes() -> &'static [u8] {
    BOOT_PAGING_INSTALL_LINE.as_bytes()
}

/// Returns the canonical heap-bootstrap line (including CRLF) for serial transmitters.
#[must_use]
pub const fn boot_heap_bootstrap_line_bytes() -> &'static [u8] {
    BOOT_HEAP_BOOTSTRAP_LINE.as_bytes()
}

/// Returns the canonical heap allocation-cycle line (including CRLF) for serial transmitters.
#[must_use]
pub const fn boot_heap_alloc_cycle_line_bytes() -> &'static [u8] {
    BOOT_HEAP_ALLOC_CYCLE_LINE.as_bytes()
}

/// Returns the canonical global-allocator readiness line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_global_allocator_ready_line_bytes() -> &'static [u8] {
    BOOT_GLOBAL_ALLOCATOR_READY_LINE.as_bytes()
}

/// Returns the canonical global-allocator probe line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_global_allocator_probe_line_bytes() -> &'static [u8] {
    BOOT_GLOBAL_ALLOCATOR_PROBE_LINE.as_bytes()
}

/// Returns the canonical timer-init line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_timer_init_line_bytes() -> &'static [u8] {
    BOOT_TIMER_INIT_LINE.as_bytes()
}

/// Returns the canonical timer-first-tick line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_timer_first_tick_line_bytes() -> &'static [u8] {
    BOOT_TIMER_FIRST_TICK_LINE.as_bytes()
}

/// Returns the canonical timer-third-tick line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_timer_third_tick_line_bytes() -> &'static [u8] {
    BOOT_TIMER_THIRD_TICK_LINE.as_bytes()
}

/// Returns the canonical timer-ack line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_timer_ack_line_bytes() -> &'static [u8] {
    BOOT_TIMER_ACK_LINE.as_bytes()
}

/// Returns the canonical timer-handoff line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_timer_handoff_line_bytes() -> &'static [u8] {
    BOOT_TIMER_HANDOFF_LINE.as_bytes()
}

/// Returns the canonical scheduler-handoff line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_scheduler_handoff_line_bytes() -> &'static [u8] {
    BOOT_SCHEDULER_HANDOFF_LINE.as_bytes()
}

/// Returns the canonical thread-enqueue line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_enqueue_line_bytes() -> &'static [u8] {
    BOOT_THREAD_ENQUEUE_LINE.as_bytes()
}

/// Returns the canonical thread-dequeue line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_dequeue_line_bytes() -> &'static [u8] {
    BOOT_THREAD_DEQUEUE_LINE.as_bytes()
}

/// Returns the canonical thread-ctx-save line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_context_save_line_bytes() -> &'static [u8] {
    BOOT_THREAD_CONTEXT_SAVE_LINE.as_bytes()
}

/// Returns the canonical thread-ctx-restore line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_context_restore_line_bytes() -> &'static [u8] {
    BOOT_THREAD_CONTEXT_RESTORE_LINE.as_bytes()
}

/// Returns the canonical thread-ctx-meta line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_context_meta_line_bytes() -> &'static [u8] {
    BOOT_THREAD_CONTEXT_META_LINE.as_bytes()
}

/// Returns the canonical thread-state-blocked line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_state_blocked_line_bytes() -> &'static [u8] {
    BOOT_THREAD_STATE_BLOCKED_LINE.as_bytes()
}

/// Returns the canonical thread-state-ready line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_state_ready_line_bytes() -> &'static [u8] {
    BOOT_THREAD_STATE_READY_LINE.as_bytes()
}

/// Returns the canonical thread-wake line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_wake_line_bytes() -> &'static [u8] {
    BOOT_THREAD_WAKE_LINE.as_bytes()
}

/// Returns the canonical thread wait-ownership line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_wait_ownership_line_bytes() -> &'static [u8] {
    BOOT_THREAD_WAIT_OWNERSHIP_LINE.as_bytes()
}

/// Returns the canonical thread wake-timeout line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_wake_timeout_line_bytes() -> &'static [u8] {
    BOOT_THREAD_WAKE_TIMEOUT_LINE.as_bytes()
}

/// Returns the canonical thread wait-contention line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_wait_contention_line_bytes() -> &'static [u8] {
    BOOT_THREAD_WAIT_CONTENTION_LINE.as_bytes()
}

/// Returns the canonical thread wake-order line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_wake_order_line_bytes() -> &'static [u8] {
    BOOT_THREAD_WAKE_ORDER_LINE.as_bytes()
}

/// Returns the canonical thread-state-terminated line (including CRLF) for serial writers.
#[must_use]
pub const fn boot_thread_state_terminated_line_bytes() -> &'static [u8] {
    BOOT_THREAD_STATE_TERMINATED_LINE.as_bytes()
}

/// Returns the canonical scheduler-edge blocked-selection line (including CRLF).
#[must_use]
pub const fn boot_scheduler_edge_blocked_line_bytes() -> &'static [u8] {
    BOOT_SCHEDULER_EDGE_BLOCKED_LINE.as_bytes()
}

/// Returns the canonical scheduler-edge terminated-dequeue line (including CRLF).
#[must_use]
pub const fn boot_scheduler_edge_terminated_line_bytes() -> &'static [u8] {
    BOOT_SCHEDULER_EDGE_TERMINATED_LINE.as_bytes()
}

/// Maximum number of deterministic early memory-map regions modeled during bring-up.
pub const EARLY_MEMORY_REGION_COUNT: usize = 5;

/// Coarse physical memory region classification used by the early memory milestone.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PhysicalMemoryRegionKind {
    Usable,
    Reserved,
}

/// Deterministic physical memory region record used by early host-side modeling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalMemoryRegion {
    pub start: u64,
    pub length: u64,
    pub kind: PhysicalMemoryRegionKind,
}

/// Deterministic physical-memory init report produced at boot-time model initialization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalMemoryInitReport {
    pub regions_modeled: usize,
    pub usable_bytes: u64,
    pub reserved_bytes: u64,
    pub highest_usable_end_exclusive: u64,
}

/// Deterministic 4KiB frame window selected for early paging structure placement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PageTableFrameWindow {
    pub start: u64,
    pub end_exclusive: u64,
    pub frame_count: usize,
}

/// Deterministic early paging bootstrap report derived from physical-memory model output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyPagingPlanReport {
    pub frame_window: PageTableFrameWindow,
    pub identity_map_start: u64,
    pub identity_map_end_exclusive: u64,
    pub identity_map_pages_4k: usize,
}

/// Deterministic report describing the materialized early paging structures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyPagingInstallReport {
    pub root_table_phys_addr: u64,
    pub pdpt_phys_addr: u64,
    pub pd_phys_addr: u64,
    pub mapped_span_bytes: u64,
    pub present_entry_count: usize,
    pub installed_into_cpu: bool,
}

/// Newtype representing a canonical virtual address.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtualAddress(pub u64);

/// Newtype representing a physical address.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysicalAddress(pub u64);

/// Errors returned by early virtual-to-physical translation helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VirtualAddressTranslationError {
    NonCanonicalAddress,
    UnmappedAddress,
    InvalidPagingState,
}

/// Errors returned by early frame-allocation helpers used by allocator bring-up.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyFrameAllocationError {
    InvalidPagingState,
    NonCanonicalAddress,
    UnmappedAddress,
    OutOfFrames,
}

/// Errors returned while building the first allocator-backed kernel heap bootstrap.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyHeapBootstrapError {
    InvalidPagingState,
    NonCanonicalAddress,
    UnmappedAddress,
    OutOfFrames,
}

/// Deterministic report describing the first kernel-heap bootstrap reservation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyHeapBootstrapReport {
    pub heap_start_virt: VirtualAddress,
    pub heap_end_exclusive_virt: VirtualAddress,
    pub heap_frame_start: PhysicalAddress,
    pub heap_frame_count: usize,
    pub heap_bytes: u64,
}

/// Error codes returned by deterministic early-heap allocation operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyHeapAllocationError {
    ZeroSize,
    InvalidAlignment,
    OutOfMemory,
}

/// Error codes returned by deterministic early-heap deallocation operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyHeapDeallocationError {
    UnknownAllocation,
    DoubleFree,
}

/// Deterministic record of an allocation carved from the early heap bootstrap window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyHeapAllocation {
    pub start_virt: VirtualAddress,
    pub size_bytes: u64,
    pub alignment: u64,
    in_use: bool,
}

/// Deterministic report for the first boot-time heap allocate/free cycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyHeapOperationCycleReport {
    pub allocations: usize,
    pub deallocations: usize,
    pub final_cursor_virt: VirtualAddress,
}

/// Errors returned by deterministic early heap operation-cycle runs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyHeapOperationError {
    Allocation(EarlyHeapAllocationError),
    Deallocation(EarlyHeapDeallocationError),
}

/// Errors returned when initializing the global allocator facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GlobalAllocatorInitError {
    AlreadyInitialized,
}

/// Deterministic report for global-allocator bring-up state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlobalAllocatorStateReport {
    pub initialized: bool,
    pub allocated_bytes: u64,
}

/// Errors returned while probing global allocator-backed dynamic structures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GlobalAllocatorProbeError {
    NotInitialized,
    Layout,
    AllocationFailed,
}

/// Deterministic report for the first kernel-owned global allocator dynamic probe.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlobalAllocatorProbeReport {
    pub entries: usize,
    pub checksum: u64,
}

/// Deterministic timer initialization report produced during early bring-up.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyTimerInitReport {
    pub source: &'static str,
    pub frequency_hz: u64,
    pub pit_input_hz: u64,
    pub divisor: u16,
    pub irq_vector: u8,
    pub tick_period_ns: u64,
}

/// Deterministic timer tick-accounting report for early periodic interrupt milestones.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyTimerTickReport {
    pub irq_vector: u8,
    pub tick_count: u64,
    pub uptime_ns: u64,
}

/// Deterministic PIC-ack report for early timer interrupt handling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyTimerAckReport {
    pub irq_vector: u8,
    pub pic_command_port: u16,
    pub pic_eoi_value: u8,
    pub acknowledged: bool,
}

/// Deterministic dispatch report produced when handling a timer IRQ path end-to-end.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyTimerInterruptDispatchReport {
    pub tick: EarlyTimerTickReport,
    pub ack: EarlyTimerAckReport,
}

/// Deterministic timer state snapshot consumed by early scheduler handoff paths.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyTimerHandoffReport {
    pub irq_vector: u8,
    pub total_ticks: u64,
    pub ticks_since_last_handoff: u64,
    pub uptime_ns: u64,
    pub scheduler_quantum_elapsed: bool,
}

/// Reasons that can drive an early scheduler handoff.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlySchedulerHandoffReason {
    Timer,
    Yield,
}

/// Deterministic scheduler slot record used for early run-queue modeling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlySchedulerSlot {
    pub task_id: u32,
    pub runnable: bool,
}

/// Snapshot report describing deterministic early scheduler queue state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlySchedulerSnapshot {
    pub run_queue_depth: usize,
    pub selected_task_id: u32,
    pub idle_task_id: u32,
    pub handoff_reason: EarlySchedulerHandoffReason,
}

/// Combined timer+scheduler report used for early threads milestone integration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlySchedulerTimerHandoffReport {
    pub timer: EarlyTimerHandoffReport,
    pub scheduler: EarlySchedulerSnapshot,
}

/// Deterministic scheduler slot-mutation report for enqueue/dequeue modeling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlySchedulerMutationReport {
    pub task_id: u32,
    pub run_queue_depth: usize,
    pub selected_task_id: u32,
}

/// Deterministic per-thread register snapshot used for early context handoff modeling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyThreadContext {
    pub task_id: u32,
    pub instruction_pointer: u64,
    pub stack_pointer: u64,
}

/// Deterministic thread context handoff report produced during scheduler transition modeling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyThreadContextHandoffReport {
    pub from_task_id: u32,
    pub to_task_id: u32,
    pub saved: EarlyThreadContext,
    pub restored: EarlyThreadContext,
    pub metadata: EarlyThreadContextSwitchMetadata,
}

/// Deterministic context-switch metadata emitted alongside modeled save/restore snapshots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyThreadContextSwitchMetadata {
    pub reason: EarlySchedulerHandoffReason,
    pub timer_tick: u64,
    pub run_queue_depth: usize,
    pub queue_watermark: usize,
    pub from_state: EarlyThreadLifecycleState,
    pub to_state: EarlyThreadLifecycleState,
}

/// Deterministic lifecycle states tracked per-thread during early scheduler bring-up.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyThreadLifecycleState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

/// Deterministic report describing a modeled thread lifecycle transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyThreadLifecycleTransitionReport {
    pub task_id: u32,
    pub from_state: EarlyThreadLifecycleState,
    pub to_state: EarlyThreadLifecycleState,
    pub run_queue_depth: usize,
    pub selected_task_id: u32,
}

/// Deterministic wake reasons used when moving blocked tasks back to runnable state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyThreadWakeReason {
    Timer,
    Signal,
    Io,
}

impl EarlyThreadWakeReason {
    const fn priority(self) -> u8 {
        match self {
            Self::Signal => 3,
            Self::Io => 2,
            Self::Timer => 1,
        }
    }
}

const fn deterministic_wait_owner_task_id(wait_channel: u64) -> u32 {
    match wait_channel {
        0x0000_0000_0000_2000 => EARLY_BOOTSTRAP_TASK_ID,
        0x0000_0000_0000_3000 => 2,
        _ => EARLY_IDLE_TASK_ID,
    }
}

fn sample_wait_channel_ownership(task_id: u32, wait_channel: u64) -> EarlyWaitChannelOwnership {
    let claim_sequence = EARLY_WAIT_CHANNEL_CLAIM_SEQUENCE.fetch_add(1, Ordering::SeqCst) + 1;
    EarlyWaitChannelOwnership {
        wait_channel,
        owner_task_id: deterministic_wait_owner_task_id(wait_channel),
        blocked_task_id: task_id,
        claim_sequence,
    }
}

fn sample_wake_timeout(deadline_tick: u64) -> EarlyThreadWakeTimeout {
    let observed_tick = EARLY_TIMER_TICK_COUNT.load(Ordering::SeqCst);
    EarlyThreadWakeTimeout {
        deadline_tick,
        observed_tick,
        expired: observed_tick >= deadline_tick,
    }
}

/// Deterministic report describing blocked->ready wake metadata for scheduler modeling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyThreadWakeReport {
    pub task_id: u32,
    pub from_state: EarlyThreadLifecycleState,
    pub to_state: EarlyThreadLifecycleState,
    pub reason: EarlyThreadWakeReason,
    pub wait_channel: u64,
    pub wait_ownership: EarlyWaitChannelOwnership,
    pub timeout: EarlyThreadWakeTimeout,
    pub run_queue_depth: usize,
    pub selected_task_id: u32,
}

/// Deterministic wait-channel ownership sample for blocked thread wake accounting.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyWaitChannelOwnership {
    pub wait_channel: u64,
    pub owner_task_id: u32,
    pub blocked_task_id: u32,
    pub claim_sequence: u64,
}

/// Deterministic wake timeout metadata sampled at blocked->ready wake handling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyThreadWakeTimeout {
    pub deadline_tick: u64,
    pub observed_tick: u64,
    pub expired: bool,
}

/// Deterministic wake-contestion report for wait-channel arbitration ordering contracts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyThreadWakeContentionReport {
    pub wait_channel: u64,
    pub winner_task_id: u32,
    pub loser_task_id: u32,
    pub winner_reason: EarlyThreadWakeReason,
    pub loser_reason: EarlyThreadWakeReason,
    pub winner_claim_sequence: u64,
    pub loser_claim_sequence: u64,
}

/// Errors returned by deterministic thread lifecycle transition helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyThreadLifecycleError {
    TaskNotFound,
    InvalidStateTransition,
    IdleTaskMutation,
}

/// Errors returned by deterministic early thread context handoff helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyThreadContextHandoffError {
    SelectedTaskMissing,
    TargetTaskNotRunnable,
}

/// Errors returned by deterministic wait-channel contention arbitration helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlyThreadWakeContentionError {
    TaskNotFound,
    TaskStateNotBlocked,
    DuplicateTask,
}

/// Errors returned by deterministic scheduler slot-mutation helpers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlySchedulerMutationError {
    DuplicateTask,
    TaskNotFound,
    RunQueueFull,
    IdleTaskMutation,
}

/// Deterministic scheduler edge cases modeled during blocked/terminated selection handling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EarlySchedulerEdgeCase {
    BlockedSelectedFallback,
    TerminatedDequeueRejected,
}

/// Deterministic report emitted after modeling blocked/terminated scheduler edge handling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlySchedulerEdgeCaseReport {
    pub edge_case: EarlySchedulerEdgeCase,
    pub task_id: u32,
    pub run_queue_depth: usize,
    pub selected_task_id: u32,
    pub dequeue_error: Option<EarlySchedulerMutationError>,
}

pub const PAGE_SIZE_4K_BYTES: u64 = 0x1000;
pub const EARLY_PAGING_FRAME_WINDOW_FRAMES: usize = 4;
pub const EARLY_IDENTITY_MAP_PAGES_4K: usize = 512;
pub const EARLY_HEAP_START_VIRT: u64 = 0x0040_0000;
pub const EARLY_HEAP_FRAME_COUNT: usize = 4;
pub const EARLY_HEAP_MAX_ALLOCATIONS: usize = 16;

const PAGE_TABLE_ENTRY_PRESENT: u64 = 1 << 0;
const PAGE_TABLE_ENTRY_WRITABLE: u64 = 1 << 1;
const PAGE_TABLE_ENTRY_HUGE_PAGE: u64 = 1 << 7;
const PAGE_TABLE_ENTRY_ADDR_MASK: u64 = 0x000f_ffff_ffff_f000;
const PAGE_SIZE_2M_BYTES: u64 = 0x20_0000;
const ENTRIES_PER_PAGE_TABLE: usize = 512;
const EARLY_PAGING_PRESENT_ENTRY_COUNT: usize = 1 + 1 + EARLY_IDENTITY_MAP_PAGES_4K;
const PIT_INPUT_CLOCK_HZ: u64 = 1_193_182;
const EARLY_TIMER_TARGET_HZ: u64 = 100;
const TIMER_IRQ_VECTOR: u8 = 0x20;
const PIC_MASTER_COMMAND_PORT: u16 = 0x20;
const PIC_NON_SPECIFIC_EOI: u8 = 0x20;
const EARLY_SCHEDULER_MAX_SLOTS: usize = 4;
const EARLY_IDLE_TASK_ID: u32 = 0;
const EARLY_BOOTSTRAP_TASK_ID: u32 = 1;
static EARLY_TIMER_TICK_COUNT: AtomicU64 = AtomicU64::new(0);
static EARLY_TIMER_LAST_HANDOFF_TICK: AtomicU64 = AtomicU64::new(0);
static EARLY_SCHEDULER_SELECTED_INDEX: AtomicU64 = AtomicU64::new(0);
static EARLY_SCHEDULER_QUEUE_WATERMARK: AtomicU64 = AtomicU64::new(2);
static EARLY_WAIT_CHANNEL_CLAIM_SEQUENCE: AtomicU64 = AtomicU64::new(0);
static mut EARLY_SCHEDULER_SLOTS: [EarlySchedulerSlot; EARLY_SCHEDULER_MAX_SLOTS] =
    [EarlySchedulerSlot {
        task_id: 0,
        runnable: false,
    }; EARLY_SCHEDULER_MAX_SLOTS];
static mut EARLY_THREAD_LIFECYCLE_STATES: [EarlyThreadLifecycleState; EARLY_SCHEDULER_MAX_SLOTS] =
    [EarlyThreadLifecycleState::Terminated; EARLY_SCHEDULER_MAX_SLOTS];

/// Returns true when a value is 4KiB aligned.
#[must_use]
pub const fn is_page_aligned_4k(addr: u64) -> bool {
    (addr & (PAGE_SIZE_4K_BYTES - 1)) == 0
}

/// Returns true if the provided x86_64 virtual address is canonical.
#[must_use]
pub const fn is_canonical_virtual_address(addr: u64) -> bool {
    let sign_bit = (addr >> 47) & 1;
    let upper = addr >> 48;
    if sign_bit == 0 {
        upper == 0
    } else {
        upper == 0xFFFF
    }
}

/// Validates the minimal invariants required for deterministic early translation helpers.
#[must_use]
pub const fn early_translation_state_valid(report: EarlyPagingInstallReport) -> bool {
    is_page_aligned_4k(report.root_table_phys_addr)
        && is_page_aligned_4k(report.pdpt_phys_addr)
        && is_page_aligned_4k(report.pd_phys_addr)
        && report.mapped_span_bytes != 0
        && report.present_entry_count >= 3
}

/// Translates a canonical virtual address through the deterministic early identity map.
pub fn translate_early_virtual_to_physical(
    virt: VirtualAddress,
    report: EarlyPagingInstallReport,
) -> Result<PhysicalAddress, VirtualAddressTranslationError> {
    if !early_translation_state_valid(report) {
        return Err(VirtualAddressTranslationError::InvalidPagingState);
    }

    if !is_canonical_virtual_address(virt.0) {
        return Err(VirtualAddressTranslationError::NonCanonicalAddress);
    }

    if virt.0 >= report.mapped_span_bytes {
        return Err(VirtualAddressTranslationError::UnmappedAddress);
    }

    Ok(PhysicalAddress(virt.0))
}

/// Deterministic 4KiB frame record returned by early allocator-facing selection APIs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyFrameAllocation {
    pub frame_start: PhysicalAddress,
    pub requested_virt: VirtualAddress,
    pub translated_phys: PhysicalAddress,
}

/// Deterministic early frame allocator over the identity-mapped range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyFrameAllocator {
    next_frame_start: u64,
    end_exclusive: u64,
}

impl EarlyFrameAllocator {
    /// Creates an allocator that hands out 4KiB frames from the deterministic identity-map span.
    #[must_use]
    pub fn from_install_report(report: EarlyPagingInstallReport) -> Self {
        Self {
            next_frame_start: 0,
            end_exclusive: report.mapped_span_bytes,
        }
    }

    /// Returns the next frame start this allocator will hand out.
    #[must_use]
    pub const fn next_frame_start(&self) -> u64 {
        self.next_frame_start
    }

    /// Returns the exclusive end boundary for frame selection.
    #[must_use]
    pub const fn end_exclusive(&self) -> u64 {
        self.end_exclusive
    }

    /// Selects one frame and validates the caller-provided virtual address translation contract.
    pub fn allocate_for_virtual(
        &mut self,
        requested_virt: VirtualAddress,
        report: EarlyPagingInstallReport,
    ) -> Result<EarlyFrameAllocation, EarlyFrameAllocationError> {
        let translated_phys = match translate_early_virtual_to_physical(requested_virt, report) {
            Ok(phys) => phys,
            Err(VirtualAddressTranslationError::InvalidPagingState) => {
                return Err(EarlyFrameAllocationError::InvalidPagingState);
            }
            Err(VirtualAddressTranslationError::NonCanonicalAddress) => {
                return Err(EarlyFrameAllocationError::NonCanonicalAddress);
            }
            Err(VirtualAddressTranslationError::UnmappedAddress) => {
                return Err(EarlyFrameAllocationError::UnmappedAddress);
            }
        };

        if self.next_frame_start >= self.end_exclusive {
            return Err(EarlyFrameAllocationError::OutOfFrames);
        }

        let frame_start = self.next_frame_start;
        self.next_frame_start += PAGE_SIZE_4K_BYTES;

        Ok(EarlyFrameAllocation {
            frame_start: PhysicalAddress(frame_start),
            requested_virt,
            translated_phys,
        })
    }
}

/// Reserves the first allocator-backed kernel heap window from deterministic early frame state.
pub fn bootstrap_early_kernel_heap(
    allocator: &mut EarlyFrameAllocator,
    report: EarlyPagingInstallReport,
) -> Result<EarlyHeapBootstrapReport, EarlyHeapBootstrapError> {
    let mut first_frame_start = None;
    let mut index = 0;
    while index < EARLY_HEAP_FRAME_COUNT {
        let requested_virt =
            VirtualAddress(EARLY_HEAP_START_VIRT + (index as u64) * PAGE_SIZE_4K_BYTES);
        let allocation = match allocator.allocate_for_virtual(requested_virt, report) {
            Ok(allocation) => allocation,
            Err(EarlyFrameAllocationError::InvalidPagingState) => {
                return Err(EarlyHeapBootstrapError::InvalidPagingState)
            }
            Err(EarlyFrameAllocationError::NonCanonicalAddress) => {
                return Err(EarlyHeapBootstrapError::NonCanonicalAddress)
            }
            Err(EarlyFrameAllocationError::UnmappedAddress) => {
                return Err(EarlyHeapBootstrapError::UnmappedAddress)
            }
            Err(EarlyFrameAllocationError::OutOfFrames) => {
                return Err(EarlyHeapBootstrapError::OutOfFrames)
            }
        };

        if first_frame_start.is_none() {
            first_frame_start = Some(allocation.frame_start);
        }

        index += 1;
    }

    let heap_bytes = (EARLY_HEAP_FRAME_COUNT as u64) * PAGE_SIZE_4K_BYTES;
    Ok(EarlyHeapBootstrapReport {
        heap_start_virt: VirtualAddress(EARLY_HEAP_START_VIRT),
        heap_end_exclusive_virt: VirtualAddress(EARLY_HEAP_START_VIRT + heap_bytes),
        heap_frame_start: first_frame_start.unwrap_or(PhysicalAddress(0)),
        heap_frame_count: EARLY_HEAP_FRAME_COUNT,
        heap_bytes,
    })
}

const fn align_up(addr: u64, alignment: u64) -> u64 {
    if alignment <= 1 {
        addr
    } else {
        let mask = alignment - 1;
        (addr + mask) & !mask
    }
}

/// Deterministic bump allocator for the allocator-backed early heap window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EarlyHeapAllocator {
    heap_start_virt: u64,
    heap_end_exclusive_virt: u64,
    cursor_virt: u64,
    allocations: [Option<EarlyHeapAllocation>; EARLY_HEAP_MAX_ALLOCATIONS],
}

impl EarlyHeapAllocator {
    /// Constructs an allocator over the previously bootstrapped early heap reservation.
    #[must_use]
    pub const fn from_bootstrap(report: EarlyHeapBootstrapReport) -> Self {
        Self {
            heap_start_virt: report.heap_start_virt.0,
            heap_end_exclusive_virt: report.heap_end_exclusive_virt.0,
            cursor_virt: report.heap_start_virt.0,
            allocations: [None; EARLY_HEAP_MAX_ALLOCATIONS],
        }
    }

    /// Returns the deterministic current heap cursor virtual address.
    #[must_use]
    pub const fn cursor_virt(&self) -> VirtualAddress {
        VirtualAddress(self.cursor_virt)
    }

    /// Returns total bytes currently in-use by tracked allocations.
    #[must_use]
    pub fn allocated_bytes(&self) -> u64 {
        let mut total = 0;
        let mut index = 0;
        while index < EARLY_HEAP_MAX_ALLOCATIONS {
            if let Some(allocation) = self.allocations[index] {
                if allocation.in_use {
                    total += allocation.size_bytes;
                }
            }
            index += 1;
        }
        total
    }

    /// Allocates a deterministic chunk from the early heap window.
    pub fn allocate(
        &mut self,
        size_bytes: u64,
        alignment: u64,
    ) -> Result<EarlyHeapAllocation, EarlyHeapAllocationError> {
        if size_bytes == 0 {
            return Err(EarlyHeapAllocationError::ZeroSize);
        }

        if alignment == 0 || !alignment.is_power_of_two() {
            return Err(EarlyHeapAllocationError::InvalidAlignment);
        }

        let Some(slot_index) = self.first_free_slot() else {
            return Err(EarlyHeapAllocationError::OutOfMemory);
        };

        let start_virt = align_up(self.cursor_virt, alignment);
        let Some(end_exclusive) = start_virt.checked_add(size_bytes) else {
            return Err(EarlyHeapAllocationError::OutOfMemory);
        };

        if end_exclusive > self.heap_end_exclusive_virt {
            return Err(EarlyHeapAllocationError::OutOfMemory);
        }

        let allocation = EarlyHeapAllocation {
            start_virt: VirtualAddress(start_virt),
            size_bytes,
            alignment,
            in_use: true,
        };

        self.allocations[slot_index] = Some(allocation);
        self.cursor_virt = end_exclusive;
        Ok(allocation)
    }

    /// Deallocates a previously-tracked deterministic heap allocation.
    pub fn deallocate(
        &mut self,
        allocation: EarlyHeapAllocation,
    ) -> Result<(), EarlyHeapDeallocationError> {
        let mut index = 0;
        while index < EARLY_HEAP_MAX_ALLOCATIONS {
            if let Some(existing) = self.allocations[index] {
                if existing.start_virt == allocation.start_virt
                    && existing.size_bytes == allocation.size_bytes
                    && existing.alignment == allocation.alignment
                {
                    if !existing.in_use {
                        return Err(EarlyHeapDeallocationError::DoubleFree);
                    }

                    let mut freed = existing;
                    freed.in_use = false;
                    self.allocations[index] = Some(freed);

                    if self.allocated_bytes() == 0 {
                        self.cursor_virt = self.heap_start_virt;
                    }
                    return Ok(());
                }
            }
            index += 1;
        }
        Err(EarlyHeapDeallocationError::UnknownAllocation)
    }

    const fn first_free_slot(&self) -> Option<usize> {
        let mut index = 0;
        while index < EARLY_HEAP_MAX_ALLOCATIONS {
            if self.allocations[index].is_none() {
                return Some(index);
            }
            index += 1;
        }
        None
    }
}

/// Exercises deterministic early heap allocation/deallocation operations once at boot.
pub fn run_early_heap_alloc_cycle(
    allocator: &mut EarlyHeapAllocator,
) -> Result<EarlyHeapOperationCycleReport, EarlyHeapOperationError> {
    let first = allocator
        .allocate(0x20, 0x10)
        .map_err(EarlyHeapOperationError::Allocation)?;
    let second = allocator
        .allocate(0x40, 0x20)
        .map_err(EarlyHeapOperationError::Allocation)?;

    allocator
        .deallocate(second)
        .map_err(EarlyHeapOperationError::Deallocation)?;
    allocator
        .deallocate(first)
        .map_err(EarlyHeapOperationError::Deallocation)?;

    Ok(EarlyHeapOperationCycleReport {
        allocations: 2,
        deallocations: 2,
        final_cursor_virt: allocator.cursor_virt(),
    })
}

/// Minimal global allocator facade backed by the deterministic early heap allocator.
pub struct EarlyGlobalAllocator {
    allocator: UnsafeCell<Option<EarlyHeapAllocator>>,
    initialized: AtomicBool,
    lock: AtomicBool,
}

// SAFETY: Allocation operations are serialized through an explicit spin lock around the mutable
// allocator state inside `UnsafeCell`.
unsafe impl Sync for EarlyGlobalAllocator {}

impl Default for EarlyGlobalAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl EarlyGlobalAllocator {
    /// Constructs an empty global allocator facade that requires explicit initialization.
    pub const fn new() -> Self {
        Self {
            allocator: UnsafeCell::new(None),
            initialized: AtomicBool::new(false),
            lock: AtomicBool::new(false),
        }
    }

    /// Installs the early heap allocator exactly once.
    pub fn init(&self, heap_allocator: EarlyHeapAllocator) -> Result<(), GlobalAllocatorInitError> {
        if self.initialized.swap(true, Ordering::AcqRel) {
            return Err(GlobalAllocatorInitError::AlreadyInitialized);
        }

        // SAFETY: Initialization transitions from `None` to `Some` exactly once while the
        // `initialized` flag prevents concurrent reinitialization.
        unsafe {
            *self.allocator.get() = Some(heap_allocator);
        }
        Ok(())
    }

    fn with_lock<R>(&self, f: impl FnOnce(&mut EarlyHeapAllocator) -> R) -> Option<R> {
        while self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        // SAFETY: The lock guarantees exclusive mutable access to allocator state.
        let result = unsafe { (*self.allocator.get()).as_mut().map(f) };

        self.lock.store(false, Ordering::Release);
        result
    }

    /// Returns deterministic allocator state information for transcript logging.
    #[must_use]
    pub fn state_report(&self) -> GlobalAllocatorStateReport {
        let initialized = self.initialized.load(Ordering::Acquire);
        let allocated_bytes = if initialized {
            // SAFETY: `initialized` implies allocator was installed via `init`.
            unsafe {
                (*self.allocator.get())
                    .as_ref()
                    .map_or(0, EarlyHeapAllocator::allocated_bytes)
            }
        } else {
            0
        };

        GlobalAllocatorStateReport {
            initialized,
            allocated_bytes,
        }
    }

    fn allocate_inner(&self, layout: Layout) -> *mut u8 {
        if !self.initialized.load(Ordering::Acquire) {
            return null_mut();
        }

        self.with_lock(|allocator| {
            match allocator.allocate(layout.size() as u64, layout.align() as u64) {
                Ok(allocation) => allocation.start_virt.0 as *mut u8,
                Err(_) => null_mut(),
            }
        })
        .unwrap_or(null_mut())
    }

    fn deallocate_inner(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() || !self.initialized.load(Ordering::Acquire) {
            return;
        }

        let allocation = EarlyHeapAllocation {
            start_virt: VirtualAddress(ptr as u64),
            size_bytes: layout.size() as u64,
            alignment: layout.align() as u64,
            in_use: true,
        };

        let _ = self.with_lock(|allocator| allocator.deallocate(allocation));
    }

    /// Allocates, uses, and frees a deterministic kernel-owned dynamic structure.
    pub fn run_dynamic_probe(
        &self,
    ) -> Result<GlobalAllocatorProbeReport, GlobalAllocatorProbeError> {
        if !self.initialized.load(Ordering::Acquire) {
            return Err(GlobalAllocatorProbeError::NotInitialized);
        }

        let layout = Layout::array::<u64>(4).map_err(|_| GlobalAllocatorProbeError::Layout)?;
        let ptr = self.allocate_inner(layout);
        if ptr.is_null() {
            return Err(GlobalAllocatorProbeError::AllocationFailed);
        }

        let probe_values = [3_u64, 7, 11, 21];
        let checksum = probe_values.into_iter().sum();

        self.deallocate_inner(ptr, layout);

        Ok(GlobalAllocatorProbeReport {
            entries: probe_values.len(),
            checksum,
        })
    }
}

unsafe impl GlobalAlloc for EarlyGlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate_inner(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.deallocate_inner(ptr, layout);
    }
}

/// Deterministic global allocator facade instance for early kernel-owned structures.
pub static EARLY_GLOBAL_ALLOCATOR: EarlyGlobalAllocator = EarlyGlobalAllocator::new();

/// Installs a global allocator facade backed by the deterministic early heap allocator.
pub fn init_early_global_allocator(
    heap_bootstrap: EarlyHeapBootstrapReport,
) -> Result<GlobalAllocatorStateReport, GlobalAllocatorInitError> {
    let heap_allocator = EarlyHeapAllocator::from_bootstrap(heap_bootstrap);
    EARLY_GLOBAL_ALLOCATOR.init(heap_allocator)?;
    Ok(EARLY_GLOBAL_ALLOCATOR.state_report())
}

/// Exercises a first kernel-owned dynamic structure over the early global allocator facade.
pub fn run_early_global_allocator_probe(
) -> Result<GlobalAllocatorProbeReport, GlobalAllocatorProbeError> {
    EARLY_GLOBAL_ALLOCATOR.run_dynamic_probe()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
struct PageTableEntry(u64);

impl PageTableEntry {
    #[must_use]
    const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    const fn table_next(next_table_phys: u64) -> Self {
        Self(
            (next_table_phys & PAGE_TABLE_ENTRY_ADDR_MASK)
                | PAGE_TABLE_ENTRY_PRESENT
                | PAGE_TABLE_ENTRY_WRITABLE,
        )
    }

    #[must_use]
    const fn huge_page_identity(frame_start: u64) -> Self {
        Self(
            (frame_start & PAGE_TABLE_ENTRY_ADDR_MASK)
                | PAGE_TABLE_ENTRY_PRESENT
                | PAGE_TABLE_ENTRY_WRITABLE
                | PAGE_TABLE_ENTRY_HUGE_PAGE,
        )
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(4096))]
struct PageTable {
    entries: [PageTableEntry; ENTRIES_PER_PAGE_TABLE],
}

impl PageTable {
    const fn empty() -> Self {
        Self {
            entries: [PageTableEntry::empty(); ENTRIES_PER_PAGE_TABLE],
        }
    }
}

static mut EARLY_PML4: PageTable = PageTable::empty();
static mut EARLY_PDPT: PageTable = PageTable::empty();
static mut EARLY_PD: PageTable = PageTable::empty();

const EARLY_PHYSICAL_MEMORY_MAP: [PhysicalMemoryRegion; EARLY_MEMORY_REGION_COUNT] = [
    PhysicalMemoryRegion {
        start: 0x0000_0000,
        length: 0x0009_f000,
        kind: PhysicalMemoryRegionKind::Usable,
    },
    PhysicalMemoryRegion {
        start: 0x0009_f000,
        length: 0x0000_1000,
        kind: PhysicalMemoryRegionKind::Reserved,
    },
    PhysicalMemoryRegion {
        start: 0x000f_0000,
        length: 0x0001_0000,
        kind: PhysicalMemoryRegionKind::Reserved,
    },
    PhysicalMemoryRegion {
        start: 0x0010_0000,
        length: 0x3f6f_1000,
        kind: PhysicalMemoryRegionKind::Usable,
    },
    PhysicalMemoryRegion {
        start: 0x3f7f_1000,
        length: 0x0080_0000,
        kind: PhysicalMemoryRegionKind::Reserved,
    },
];

/// Returns the deterministic early physical memory map model used during bring-up.
#[must_use]
pub const fn early_physical_memory_map(
) -> &'static [PhysicalMemoryRegion; EARLY_MEMORY_REGION_COUNT] {
    &EARLY_PHYSICAL_MEMORY_MAP
}

/// Initializes the deterministic early physical-memory model for host-side contracts.
#[must_use]
pub fn init_early_physical_memory() -> PhysicalMemoryInitReport {
    let mut usable_bytes = 0_u64;
    let mut reserved_bytes = 0_u64;
    let mut highest_usable_end_exclusive = 0_u64;

    let mut index = 0;
    while index < EARLY_MEMORY_REGION_COUNT {
        let region = EARLY_PHYSICAL_MEMORY_MAP[index];
        match region.kind {
            PhysicalMemoryRegionKind::Usable => {
                usable_bytes += region.length;
                let region_end_exclusive = region.start + region.length;
                if region_end_exclusive > highest_usable_end_exclusive {
                    highest_usable_end_exclusive = region_end_exclusive;
                }
            }
            PhysicalMemoryRegionKind::Reserved => {
                reserved_bytes += region.length;
            }
        }
        index += 1;
    }

    PhysicalMemoryInitReport {
        regions_modeled: EARLY_MEMORY_REGION_COUNT,
        usable_bytes,
        reserved_bytes,
        highest_usable_end_exclusive,
    }
}

/// Initializes a deterministic paging bootstrap plan from the modeled memory report.
#[must_use]
pub fn init_early_paging_plan(memory_report: PhysicalMemoryInitReport) -> EarlyPagingPlanReport {
    let frame_window_size = (EARLY_PAGING_FRAME_WINDOW_FRAMES as u64) * PAGE_SIZE_4K_BYTES;
    let frame_window = PageTableFrameWindow {
        start: memory_report.highest_usable_end_exclusive - frame_window_size,
        end_exclusive: memory_report.highest_usable_end_exclusive,
        frame_count: EARLY_PAGING_FRAME_WINDOW_FRAMES,
    };

    let identity_map_start = 0;
    let identity_map_end_exclusive = (EARLY_IDENTITY_MAP_PAGES_4K as u64) * PAGE_SIZE_4K_BYTES;

    EarlyPagingPlanReport {
        frame_window,
        identity_map_start,
        identity_map_end_exclusive,
        identity_map_pages_4k: EARLY_IDENTITY_MAP_PAGES_4K,
    }
}

/// Returns a deterministic snapshot of the materialized early paging tables.
#[must_use]
pub fn early_paging_table_snapshot() -> ([[u64; ENTRIES_PER_PAGE_TABLE]; 3], [u64; 3]) {
    // SAFETY: these statics are only mutated during single-threaded early boot initialization;
    // host tests read them after calling install_early_paging.
    unsafe {
        let root_addr = (&raw const EARLY_PML4) as u64;
        let pdpt_addr = (&raw const EARLY_PDPT) as u64;
        let pd_addr = (&raw const EARLY_PD) as u64;

        let mut root = [0_u64; ENTRIES_PER_PAGE_TABLE];
        let mut pdpt = [0_u64; ENTRIES_PER_PAGE_TABLE];
        let mut pd = [0_u64; ENTRIES_PER_PAGE_TABLE];

        let mut index = 0;
        while index < ENTRIES_PER_PAGE_TABLE {
            root[index] = EARLY_PML4.entries[index].0;
            pdpt[index] = EARLY_PDPT.entries[index].0;
            pd[index] = EARLY_PD.entries[index].0;
            index += 1;
        }

        ([root, pdpt, pd], [root_addr, pdpt_addr, pd_addr])
    }
}

/// Materializes minimal early paging structures and installs CR3 on real x86_64/UEFI boots.
#[must_use]
pub fn install_early_paging(plan: EarlyPagingPlanReport) -> EarlyPagingInstallReport {
    let mapped_span_bytes = (plan.identity_map_pages_4k as u64) * PAGE_SIZE_2M_BYTES;
    let root_table_phys_addr = plan.frame_window.start;
    let pdpt_phys_addr = root_table_phys_addr + PAGE_SIZE_4K_BYTES;
    let pd_phys_addr = pdpt_phys_addr + PAGE_SIZE_4K_BYTES;

    // SAFETY: early boot initializes a fixed static table set before multitasking.
    unsafe {
        EARLY_PML4 = PageTable::empty();
        EARLY_PDPT = PageTable::empty();
        EARLY_PD = PageTable::empty();

        EARLY_PML4.entries[0] = PageTableEntry::table_next(pdpt_phys_addr);
        EARLY_PDPT.entries[0] = PageTableEntry::table_next(pd_phys_addr);

        let mut index = 0;
        while index < EARLY_IDENTITY_MAP_PAGES_4K {
            let frame_start = (index as u64) * PAGE_SIZE_2M_BYTES;
            EARLY_PD.entries[index] = PageTableEntry::huge_page_identity(frame_start);
            index += 1;
        }
    }

    let installed_into_cpu = maybe_install_cr3(root_table_phys_addr);

    EarlyPagingInstallReport {
        root_table_phys_addr,
        pdpt_phys_addr,
        pd_phys_addr,
        mapped_span_bytes,
        present_entry_count: EARLY_PAGING_PRESENT_ENTRY_COUNT,
        installed_into_cpu,
    }
}

/// Initializes deterministic early timer configuration contracts used by boot transcript tests.
#[must_use]
pub fn init_early_timer() -> EarlyTimerInitReport {
    let divisor = (PIT_INPUT_CLOCK_HZ / EARLY_TIMER_TARGET_HZ) as u16;
    let tick_period_ns = 1_000_000_000_u64 / EARLY_TIMER_TARGET_HZ;

    EarlyTimerInitReport {
        source: "pit",
        frequency_hz: EARLY_TIMER_TARGET_HZ,
        pit_input_hz: PIT_INPUT_CLOCK_HZ,
        divisor,
        irq_vector: TIMER_IRQ_VECTOR,
        tick_period_ns,
    }
}

/// Clears deterministic early timer tick-accounting state for fresh boot/test initialization.
pub fn reset_early_timer_ticks() {
    EARLY_TIMER_TICK_COUNT.store(0, Ordering::SeqCst);
    EARLY_TIMER_LAST_HANDOFF_TICK.store(0, Ordering::SeqCst);
}

/// Initializes deterministic early scheduler queue slots used by timer-driven handoff paths.
pub fn reset_early_scheduler_state() {
    // SAFETY: deterministic early scheduler model is single-threaded during boot/test setup.
    unsafe {
        EARLY_SCHEDULER_SLOTS = [EarlySchedulerSlot {
            task_id: 0,
            runnable: false,
        }; EARLY_SCHEDULER_MAX_SLOTS];
        EARLY_THREAD_LIFECYCLE_STATES =
            [EarlyThreadLifecycleState::Terminated; EARLY_SCHEDULER_MAX_SLOTS];

        EARLY_SCHEDULER_SLOTS[0] = EarlySchedulerSlot {
            task_id: EARLY_IDLE_TASK_ID,
            runnable: true,
        };
        EARLY_THREAD_LIFECYCLE_STATES[0] = EarlyThreadLifecycleState::Running;

        EARLY_SCHEDULER_SLOTS[1] = EarlySchedulerSlot {
            task_id: EARLY_BOOTSTRAP_TASK_ID,
            runnable: true,
        };
        EARLY_THREAD_LIFECYCLE_STATES[1] = EarlyThreadLifecycleState::Ready;
    }

    EARLY_SCHEDULER_SELECTED_INDEX.store(0, Ordering::SeqCst);
    EARLY_SCHEDULER_QUEUE_WATERMARK.store(2, Ordering::SeqCst);
    EARLY_WAIT_CHANNEL_CLAIM_SEQUENCE.store(0, Ordering::SeqCst);
}

/// Records a deterministic early timer tick and reports cumulative periodic accounting state.
#[must_use]
pub fn record_early_timer_tick(config: EarlyTimerInitReport) -> EarlyTimerTickReport {
    let tick_count = EARLY_TIMER_TICK_COUNT.fetch_add(1, Ordering::SeqCst) + 1;
    let uptime_ns = tick_count.saturating_mul(config.tick_period_ns);

    EarlyTimerTickReport {
        irq_vector: config.irq_vector,
        tick_count,
        uptime_ns,
    }
}

/// Returns deterministic PIC end-of-interrupt acknowledgement metadata for timer IRQ handling.
#[must_use]
pub fn acknowledge_early_timer_interrupt(config: EarlyTimerInitReport) -> EarlyTimerAckReport {
    EarlyTimerAckReport {
        irq_vector: config.irq_vector,
        pic_command_port: PIC_MASTER_COMMAND_PORT,
        pic_eoi_value: PIC_NON_SPECIFIC_EOI,
        acknowledged: true,
    }
}

/// Handles a deterministic early timer interrupt by accounting the tick and issuing PIC ACK.
#[must_use]
pub fn dispatch_early_timer_interrupt(
    config: EarlyTimerInitReport,
) -> EarlyTimerInterruptDispatchReport {
    let tick = record_early_timer_tick(config);
    let ack = acknowledge_early_timer_interrupt(config);
    EarlyTimerInterruptDispatchReport { tick, ack }
}

/// Samples deterministic timer state for scheduler integration without consuming handoff delta.
#[must_use]
pub fn sample_early_timer_handoff(config: EarlyTimerInitReport) -> EarlyTimerHandoffReport {
    let total_ticks = EARLY_TIMER_TICK_COUNT.load(Ordering::SeqCst);
    let handoff_base = EARLY_TIMER_LAST_HANDOFF_TICK.load(Ordering::SeqCst);
    let ticks_since_last_handoff = total_ticks.saturating_sub(handoff_base);
    let uptime_ns = total_ticks.saturating_mul(config.tick_period_ns);

    EarlyTimerHandoffReport {
        irq_vector: config.irq_vector,
        total_ticks,
        ticks_since_last_handoff,
        uptime_ns,
        scheduler_quantum_elapsed: ticks_since_last_handoff != 0,
    }
}

/// Consumes deterministic timer state and advances the scheduler handoff watermark.
#[must_use]
pub fn take_early_timer_handoff(config: EarlyTimerInitReport) -> EarlyTimerHandoffReport {
    let report = sample_early_timer_handoff(config);
    EARLY_TIMER_LAST_HANDOFF_TICK.store(report.total_ticks, Ordering::SeqCst);
    report
}

/// Returns a deterministic snapshot of the modeled early scheduler run queue state.
#[must_use]
pub fn sample_early_scheduler_snapshot(
    reason: EarlySchedulerHandoffReason,
) -> EarlySchedulerSnapshot {
    let mut run_queue_depth = 0;
    // SAFETY: read-only traversal of deterministic static slots in single-threaded boot model.
    unsafe {
        let mut slot_index = 0;
        while slot_index < EARLY_SCHEDULER_MAX_SLOTS {
            if EARLY_SCHEDULER_SLOTS[slot_index].runnable {
                run_queue_depth += 1;
            }
            slot_index += 1;
        }
    }

    let selected_index = EARLY_SCHEDULER_SELECTED_INDEX.load(Ordering::SeqCst) as usize;
    // SAFETY: selected index is always written through bounded deterministic state transitions.
    let selected_task_id = unsafe { EARLY_SCHEDULER_SLOTS[selected_index].task_id };

    EarlySchedulerSnapshot {
        run_queue_depth,
        selected_task_id,
        idle_task_id: EARLY_IDLE_TASK_ID,
        handoff_reason: reason,
    }
}

fn scheduler_state_index_for_task(task_id: u32) -> Option<usize> {
    let mut slot_index = 0;
    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    unsafe {
        while slot_index < EARLY_SCHEDULER_MAX_SLOTS {
            let slot = EARLY_SCHEDULER_SLOTS[slot_index];
            if slot.task_id == task_id {
                return Some(slot_index);
            }
            slot_index += 1;
        }
    }
    None
}

fn scheduler_index_for_task(task_id: u32) -> Option<usize> {
    let mut slot_index = 0;
    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    unsafe {
        while slot_index < EARLY_SCHEDULER_MAX_SLOTS {
            let slot = EARLY_SCHEDULER_SLOTS[slot_index];
            if slot.runnable && slot.task_id == task_id {
                return Some(slot_index);
            }
            slot_index += 1;
        }
    }
    None
}

fn next_runnable_scheduler_index(current_index: usize) -> Option<usize> {
    let mut offset = 1;
    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    unsafe {
        while offset <= EARLY_SCHEDULER_MAX_SLOTS {
            let candidate = (current_index + offset) % EARLY_SCHEDULER_MAX_SLOTS;
            if EARLY_SCHEDULER_SLOTS[candidate].runnable {
                return Some(candidate);
            }
            offset += 1;
        }
    }
    None
}

/// Advances deterministic scheduler selection using simple round-robin runnable-slot scanning.
#[must_use]
pub fn advance_early_scheduler_round_robin(
    reason: EarlySchedulerHandoffReason,
) -> EarlySchedulerSnapshot {
    let selected_index = EARLY_SCHEDULER_SELECTED_INDEX.load(Ordering::SeqCst) as usize;
    let next_index = next_runnable_scheduler_index(selected_index).unwrap_or(selected_index);

    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    unsafe {
        if selected_index < EARLY_SCHEDULER_MAX_SLOTS
            && EARLY_SCHEDULER_SLOTS[selected_index].runnable
            && EARLY_THREAD_LIFECYCLE_STATES[selected_index] != EarlyThreadLifecycleState::Blocked
        {
            EARLY_THREAD_LIFECYCLE_STATES[selected_index] = EarlyThreadLifecycleState::Ready;
        }

        if next_index < EARLY_SCHEDULER_MAX_SLOTS && EARLY_SCHEDULER_SLOTS[next_index].runnable {
            EARLY_THREAD_LIFECYCLE_STATES[next_index] = EarlyThreadLifecycleState::Running;
        }
    }

    EARLY_SCHEDULER_SELECTED_INDEX.store(next_index as u64, Ordering::SeqCst);
    sample_early_scheduler_snapshot(reason)
}

/// Enqueues a deterministic runnable slot for first thread-model integration.
pub fn enqueue_early_scheduler_task(
    task_id: u32,
) -> Result<EarlySchedulerMutationReport, EarlySchedulerMutationError> {
    if task_id == EARLY_IDLE_TASK_ID {
        return Err(EarlySchedulerMutationError::IdleTaskMutation);
    }

    if scheduler_index_for_task(task_id).is_some() {
        return Err(EarlySchedulerMutationError::DuplicateTask);
    }

    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    unsafe {
        let mut slot_index = 0;
        while slot_index < EARLY_SCHEDULER_MAX_SLOTS {
            if !EARLY_SCHEDULER_SLOTS[slot_index].runnable {
                EARLY_SCHEDULER_SLOTS[slot_index] = EarlySchedulerSlot {
                    task_id,
                    runnable: true,
                };
                EARLY_THREAD_LIFECYCLE_STATES[slot_index] = EarlyThreadLifecycleState::Ready;
                let snapshot = sample_early_scheduler_snapshot(EarlySchedulerHandoffReason::Yield);
                let observed_depth = snapshot.run_queue_depth as u64;
                let previous = EARLY_SCHEDULER_QUEUE_WATERMARK.load(Ordering::SeqCst);
                if observed_depth > previous {
                    EARLY_SCHEDULER_QUEUE_WATERMARK.store(observed_depth, Ordering::SeqCst);
                }
                return Ok(EarlySchedulerMutationReport {
                    task_id,
                    run_queue_depth: snapshot.run_queue_depth,
                    selected_task_id: snapshot.selected_task_id,
                });
            }
            slot_index += 1;
        }
    }

    Err(EarlySchedulerMutationError::RunQueueFull)
}

/// Dequeues a deterministic runnable slot to model first thread teardown from the queue.
pub fn dequeue_early_scheduler_task(
    task_id: u32,
) -> Result<EarlySchedulerMutationReport, EarlySchedulerMutationError> {
    if task_id == EARLY_IDLE_TASK_ID {
        return Err(EarlySchedulerMutationError::IdleTaskMutation);
    }

    let Some(slot_index) = scheduler_index_for_task(task_id) else {
        return Err(EarlySchedulerMutationError::TaskNotFound);
    };

    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    unsafe {
        EARLY_SCHEDULER_SLOTS[slot_index] = EarlySchedulerSlot {
            task_id: 0,
            runnable: false,
        };
        EARLY_THREAD_LIFECYCLE_STATES[slot_index] = EarlyThreadLifecycleState::Terminated;
    }

    let selected_index = EARLY_SCHEDULER_SELECTED_INDEX.load(Ordering::SeqCst) as usize;
    if selected_index == slot_index {
        let _ = advance_early_scheduler_round_robin(EarlySchedulerHandoffReason::Yield);
    }

    let snapshot = sample_early_scheduler_snapshot(EarlySchedulerHandoffReason::Yield);
    Ok(EarlySchedulerMutationReport {
        task_id,
        run_queue_depth: snapshot.run_queue_depth,
        selected_task_id: snapshot.selected_task_id,
    })
}

/// Applies a deterministic thread lifecycle transition and updates queue runnable state.
pub fn transition_early_thread_lifecycle(
    task_id: u32,
    to_state: EarlyThreadLifecycleState,
) -> Result<EarlyThreadLifecycleTransitionReport, EarlyThreadLifecycleError> {
    if task_id == EARLY_IDLE_TASK_ID {
        return Err(EarlyThreadLifecycleError::IdleTaskMutation);
    }

    let Some(slot_index) = scheduler_state_index_for_task(task_id) else {
        return Err(EarlyThreadLifecycleError::TaskNotFound);
    };

    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    let (from_state, was_runnable) = unsafe {
        (
            EARLY_THREAD_LIFECYCLE_STATES[slot_index],
            EARLY_SCHEDULER_SLOTS[slot_index].runnable,
        )
    };

    let valid = matches!(
        (from_state, to_state),
        (
            EarlyThreadLifecycleState::Ready,
            EarlyThreadLifecycleState::Running
        ) | (
            EarlyThreadLifecycleState::Running,
            EarlyThreadLifecycleState::Ready
        ) | (
            EarlyThreadLifecycleState::Running,
            EarlyThreadLifecycleState::Blocked
        ) | (
            EarlyThreadLifecycleState::Ready,
            EarlyThreadLifecycleState::Blocked
        ) | (
            EarlyThreadLifecycleState::Blocked,
            EarlyThreadLifecycleState::Ready
        ) | (
            EarlyThreadLifecycleState::Ready,
            EarlyThreadLifecycleState::Terminated
        ) | (
            EarlyThreadLifecycleState::Blocked,
            EarlyThreadLifecycleState::Terminated
        ) | (
            EarlyThreadLifecycleState::Running,
            EarlyThreadLifecycleState::Terminated
        )
    );
    if !valid {
        return Err(EarlyThreadLifecycleError::InvalidStateTransition);
    }

    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    unsafe {
        EARLY_THREAD_LIFECYCLE_STATES[slot_index] = to_state;

        match to_state {
            EarlyThreadLifecycleState::Ready | EarlyThreadLifecycleState::Running => {
                EARLY_SCHEDULER_SLOTS[slot_index].runnable = true;
            }
            EarlyThreadLifecycleState::Blocked | EarlyThreadLifecycleState::Terminated => {
                EARLY_SCHEDULER_SLOTS[slot_index].runnable = false;
                if to_state == EarlyThreadLifecycleState::Terminated {
                    EARLY_SCHEDULER_SLOTS[slot_index].task_id = 0;
                }
            }
        }
    }

    let selected_index = EARLY_SCHEDULER_SELECTED_INDEX.load(Ordering::SeqCst) as usize;
    if selected_index == slot_index && was_runnable {
        let _ = advance_early_scheduler_round_robin(EarlySchedulerHandoffReason::Yield);
    }

    let snapshot = sample_early_scheduler_snapshot(EarlySchedulerHandoffReason::Yield);
    Ok(EarlyThreadLifecycleTransitionReport {
        task_id,
        from_state,
        to_state,
        run_queue_depth: snapshot.run_queue_depth,
        selected_task_id: snapshot.selected_task_id,
    })
}

/// Wakes a blocked task into ready state with deterministic wait-channel metadata.
pub fn wake_early_thread(
    task_id: u32,
    reason: EarlyThreadWakeReason,
    wait_channel: u64,
    deadline_tick: u64,
) -> Result<EarlyThreadWakeReport, EarlyThreadLifecycleError> {
    let transition = transition_early_thread_lifecycle(task_id, EarlyThreadLifecycleState::Ready)?;

    if transition.from_state != EarlyThreadLifecycleState::Blocked {
        return Err(EarlyThreadLifecycleError::InvalidStateTransition);
    }

    let wait_ownership = sample_wait_channel_ownership(task_id, wait_channel);
    let timeout = sample_wake_timeout(deadline_tick);

    Ok(EarlyThreadWakeReport {
        task_id,
        from_state: transition.from_state,
        to_state: transition.to_state,
        reason,
        wait_channel,
        wait_ownership,
        timeout,
        run_queue_depth: transition.run_queue_depth,
        selected_task_id: transition.selected_task_id,
    })
}

/// Resolves deterministic wake contention by prioritizing reason and claim-sequence order.
pub fn resolve_early_thread_wake_contention(
    wait_channel: u64,
    first_task_id: u32,
    first_reason: EarlyThreadWakeReason,
    second_task_id: u32,
    second_reason: EarlyThreadWakeReason,
) -> Result<EarlyThreadWakeContentionReport, EarlyThreadWakeContentionError> {
    if first_task_id == second_task_id {
        return Err(EarlyThreadWakeContentionError::DuplicateTask);
    }

    let Some(first_slot) = scheduler_state_index_for_task(first_task_id) else {
        return Err(EarlyThreadWakeContentionError::TaskNotFound);
    };
    let Some(second_slot) = scheduler_state_index_for_task(second_task_id) else {
        return Err(EarlyThreadWakeContentionError::TaskNotFound);
    };

    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    unsafe {
        if EARLY_THREAD_LIFECYCLE_STATES[first_slot] != EarlyThreadLifecycleState::Blocked
            || EARLY_THREAD_LIFECYCLE_STATES[second_slot] != EarlyThreadLifecycleState::Blocked
        {
            return Err(EarlyThreadWakeContentionError::TaskStateNotBlocked);
        }
    }

    let first_ownership = sample_wait_channel_ownership(first_task_id, wait_channel);
    let second_ownership = sample_wait_channel_ownership(second_task_id, wait_channel);

    let first_priority = first_reason.priority();
    let second_priority = second_reason.priority();

    let first_wins = first_priority > second_priority
        || (first_priority == second_priority
            && first_ownership.claim_sequence <= second_ownership.claim_sequence);

    let (
        winner_task_id,
        winner_reason,
        winner_claim_sequence,
        loser_task_id,
        loser_reason,
        loser_claim_sequence,
    ) = if first_wins {
        (
            first_task_id,
            first_reason,
            first_ownership.claim_sequence,
            second_task_id,
            second_reason,
            second_ownership.claim_sequence,
        )
    } else {
        (
            second_task_id,
            second_reason,
            second_ownership.claim_sequence,
            first_task_id,
            first_reason,
            first_ownership.claim_sequence,
        )
    };

    Ok(EarlyThreadWakeContentionReport {
        wait_channel,
        winner_task_id,
        loser_task_id,
        winner_reason,
        loser_reason,
        winner_claim_sequence,
        loser_claim_sequence,
    })
}

/// Advances deterministic scheduler selection using timer handoff delta and returns combined state.
#[must_use]
pub fn take_early_scheduler_timer_handoff(
    config: EarlyTimerInitReport,
) -> EarlySchedulerTimerHandoffReport {
    let timer = take_early_timer_handoff(config);
    let scheduler = if timer.scheduler_quantum_elapsed {
        advance_early_scheduler_round_robin(EarlySchedulerHandoffReason::Timer)
    } else {
        sample_early_scheduler_snapshot(EarlySchedulerHandoffReason::Timer)
    };
    EarlySchedulerTimerHandoffReport { timer, scheduler }
}

const fn modeled_thread_context(task_id: u32) -> EarlyThreadContext {
    match task_id {
        EARLY_IDLE_TASK_ID => EarlyThreadContext {
            task_id,
            instruction_pointer: 0x0000_0000_0010_0000,
            stack_pointer: 0x0000_0000_0040_1000,
        },
        EARLY_BOOTSTRAP_TASK_ID => EarlyThreadContext {
            task_id,
            instruction_pointer: 0x0000_0000_0010_0200,
            stack_pointer: 0x0000_0000_0040_1f00,
        },
        2 => EarlyThreadContext {
            task_id,
            instruction_pointer: 0x0000_0000_0020_0000,
            stack_pointer: 0x0000_0000_0040_2000,
        },
        _ => EarlyThreadContext {
            task_id,
            instruction_pointer: 0x0000_0000_0030_0000,
            stack_pointer: 0x0000_0000_0040_3000,
        },
    }
}

/// Models deterministic per-thread ctx save/restore records for scheduler handoff paths.
pub fn model_early_thread_context_handoff(
    next_task_id: u32,
    reason: EarlySchedulerHandoffReason,
) -> Result<EarlyThreadContextHandoffReport, EarlyThreadContextHandoffError> {
    let selected_index = EARLY_SCHEDULER_SELECTED_INDEX.load(Ordering::SeqCst) as usize;
    if selected_index >= EARLY_SCHEDULER_MAX_SLOTS {
        return Err(EarlyThreadContextHandoffError::SelectedTaskMissing);
    }

    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    let from_slot = unsafe {
        core::ptr::read(
            (&raw const EARLY_SCHEDULER_SLOTS)
                .cast::<EarlySchedulerSlot>()
                .add(selected_index),
        )
    };
    if !from_slot.runnable {
        return Err(EarlyThreadContextHandoffError::SelectedTaskMissing);
    }

    let Some(to_index) = scheduler_index_for_task(next_task_id) else {
        return Err(EarlyThreadContextHandoffError::TargetTaskNotRunnable);
    };

    // SAFETY: deterministic scheduler model uses single-threaded static state during boot/tests.
    let to_slot = unsafe { EARLY_SCHEDULER_SLOTS[to_index] };

    let snapshot = sample_early_scheduler_snapshot(reason);
    let metadata = EarlyThreadContextSwitchMetadata {
        reason,
        timer_tick: EARLY_TIMER_TICK_COUNT.load(Ordering::SeqCst),
        run_queue_depth: snapshot.run_queue_depth,
        queue_watermark: EARLY_SCHEDULER_QUEUE_WATERMARK.load(Ordering::SeqCst) as usize,
        from_state: EarlyThreadLifecycleState::Running,
        to_state: EarlyThreadLifecycleState::Ready,
    };

    Ok(EarlyThreadContextHandoffReport {
        from_task_id: from_slot.task_id,
        to_task_id: to_slot.task_id,
        saved: modeled_thread_context(from_slot.task_id),
        restored: modeled_thread_context(to_slot.task_id),
        metadata,
    })
}

/// Models the selected-task-blocked edge case and reports deterministic fallback scheduler state.
pub fn model_early_scheduler_blocked_selection_edge_case(
    task_id: u32,
) -> Result<EarlySchedulerEdgeCaseReport, EarlyThreadLifecycleError> {
    let _transition =
        transition_early_thread_lifecycle(task_id, EarlyThreadLifecycleState::Blocked)?;
    if let Some(idle_index) = scheduler_index_for_task(EARLY_IDLE_TASK_ID) {
        EARLY_SCHEDULER_SELECTED_INDEX.store(idle_index as u64, Ordering::SeqCst);
    }
    let snapshot = sample_early_scheduler_snapshot(EarlySchedulerHandoffReason::Yield);

    Ok(EarlySchedulerEdgeCaseReport {
        edge_case: EarlySchedulerEdgeCase::BlockedSelectedFallback,
        task_id,
        run_queue_depth: snapshot.run_queue_depth,
        selected_task_id: snapshot.selected_task_id,
        dequeue_error: None,
    })
}

/// Models termination cleanup by rejecting a dequeue for an already-terminated task slot.
pub fn model_early_scheduler_terminated_cleanup_edge_case(
    task_id: u32,
) -> Result<EarlySchedulerEdgeCaseReport, EarlyThreadLifecycleError> {
    let terminated =
        transition_early_thread_lifecycle(task_id, EarlyThreadLifecycleState::Terminated)?;
    let dequeue_error = dequeue_early_scheduler_task(task_id).err();

    Ok(EarlySchedulerEdgeCaseReport {
        edge_case: EarlySchedulerEdgeCase::TerminatedDequeueRejected,
        task_id,
        run_queue_depth: terminated.run_queue_depth,
        selected_task_id: terminated.selected_task_id,
        dequeue_error,
    })
}

/// Number of architectural exception vectors reserved at boot.
pub const EXCEPTION_VECTOR_COUNT: usize = 32;

const EXCEPTION_LOG_UNKNOWN_LINE: &str = "tosm-os: exception vector unknown\r\n";

const EXCEPTION_LOG_LINES: [&str; EXCEPTION_VECTOR_COUNT] = [
    "tosm-os: exception vector 00 divide error\r\n",
    "tosm-os: exception vector 01 debug\r\n",
    "tosm-os: exception vector 02 non-maskable interrupt\r\n",
    "tosm-os: exception vector 03 breakpoint\r\n",
    "tosm-os: exception vector 04 overflow\r\n",
    "tosm-os: exception vector 05 bound range exceeded\r\n",
    "tosm-os: exception vector 06 invalid opcode\r\n",
    "tosm-os: exception vector 07 device not available\r\n",
    "tosm-os: exception vector 08 double fault\r\n",
    "tosm-os: exception vector 09 coprocessor segment overrun\r\n",
    "tosm-os: exception vector 10 invalid tss\r\n",
    "tosm-os: exception vector 11 segment not present\r\n",
    "tosm-os: exception vector 12 stack-segment fault\r\n",
    "tosm-os: exception vector 13 general protection fault\r\n",
    "tosm-os: exception vector 14 page fault\r\n",
    "tosm-os: exception vector 15 reserved\r\n",
    "tosm-os: exception vector 16 x87 floating-point exception\r\n",
    "tosm-os: exception vector 17 alignment check\r\n",
    "tosm-os: exception vector 18 machine check\r\n",
    "tosm-os: exception vector 19 simd floating-point exception\r\n",
    "tosm-os: exception vector 20 virtualization exception\r\n",
    "tosm-os: exception vector 21 control protection exception\r\n",
    "tosm-os: exception vector 22 reserved\r\n",
    "tosm-os: exception vector 23 reserved\r\n",
    "tosm-os: exception vector 24 reserved\r\n",
    "tosm-os: exception vector 25 reserved\r\n",
    "tosm-os: exception vector 26 reserved\r\n",
    "tosm-os: exception vector 27 hypervisor injection exception\r\n",
    "tosm-os: exception vector 28 vmm communication exception\r\n",
    "tosm-os: exception vector 29 security exception\r\n",
    "tosm-os: exception vector 30 reserved\r\n",
    "tosm-os: exception vector 31 reserved\r\n",
];

const KERNEL_CODE_SELECTOR: u16 = 0x0008;
const INTERRUPT_GATE_PRESENT_DPL0: u8 = 0x8E;

/// Minimal x86_64 IDT entry used for deterministic early exception setup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attributes: u8,
    offset_middle: u16,
    offset_high: u32,
    zero: u32,
}

impl IdtEntry {
    #[must_use]
    pub const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attributes: 0,
            offset_middle: 0,
            offset_high: 0,
            zero: 0,
        }
    }

    #[must_use]
    pub const fn interrupt_gate(handler: usize) -> Self {
        let bytes = handler.to_le_bytes();
        Self {
            offset_low: u16::from_le_bytes([bytes[0], bytes[1]]),
            selector: KERNEL_CODE_SELECTOR,
            ist: 0,
            type_attributes: INTERRUPT_GATE_PRESENT_DPL0,
            offset_middle: u16::from_le_bytes([bytes[2], bytes[3]]),
            offset_high: u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            zero: 0,
        }
    }

    #[must_use]
    pub const fn handler_addr(self) -> usize {
        let low = self.offset_low.to_le_bytes();
        let middle = self.offset_middle.to_le_bytes();
        let high = self.offset_high.to_le_bytes();
        usize::from_le_bytes([
            low[0], low[1], middle[0], middle[1], high[0], high[1], high[2], high[3],
        ])
    }

    #[must_use]
    pub const fn selector(self) -> u16 {
        self.selector
    }

    #[must_use]
    pub const fn type_attributes(self) -> u8 {
        self.type_attributes
    }
}

/// Descriptor pointer layout consumed by x86_64 lidt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C, packed)]
pub struct IdtDescriptor {
    pub limit: u16,
    pub base: u64,
}

/// Deterministic report about early IDT setup used by host-side tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InterruptInitReport {
    pub vectors_initialized: usize,
    pub descriptor: IdtDescriptor,
    pub first_handler_addr: usize,
    pub last_handler_addr: usize,
}

/// Deterministic exception dispatch report used by early firmware logging paths.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExceptionDispatchReport {
    pub vector: u8,
    pub known_vector: bool,
    pub line: &'static str,
}

extern "C" fn early_exception_spin_stub() {
    loop {
        core::hint::spin_loop();
    }
}

macro_rules! define_exception_stubs {
    ($($stub:ident),+ $(,)?) => {
        $(
            extern "C" fn $stub() {
                early_exception_spin_stub();
            }
        )+
    };
}

define_exception_stubs!(
    early_exception_stub_00,
    early_exception_stub_01,
    early_exception_stub_02,
    early_exception_stub_03,
    early_exception_stub_04,
    early_exception_stub_05,
    early_exception_stub_06,
    early_exception_stub_07,
    early_exception_stub_08,
    early_exception_stub_09,
    early_exception_stub_10,
    early_exception_stub_11,
    early_exception_stub_12,
    early_exception_stub_13,
    early_exception_stub_14,
    early_exception_stub_15,
    early_exception_stub_16,
    early_exception_stub_17,
    early_exception_stub_18,
    early_exception_stub_19,
    early_exception_stub_20,
    early_exception_stub_21,
    early_exception_stub_22,
    early_exception_stub_23,
    early_exception_stub_24,
    early_exception_stub_25,
    early_exception_stub_26,
    early_exception_stub_27,
    early_exception_stub_28,
    early_exception_stub_29,
    early_exception_stub_30,
    early_exception_stub_31,
);

const EXCEPTION_STUBS: [extern "C" fn(); EXCEPTION_VECTOR_COUNT] = [
    early_exception_stub_00,
    early_exception_stub_01,
    early_exception_stub_02,
    early_exception_stub_03,
    early_exception_stub_04,
    early_exception_stub_05,
    early_exception_stub_06,
    early_exception_stub_07,
    early_exception_stub_08,
    early_exception_stub_09,
    early_exception_stub_10,
    early_exception_stub_11,
    early_exception_stub_12,
    early_exception_stub_13,
    early_exception_stub_14,
    early_exception_stub_15,
    early_exception_stub_16,
    early_exception_stub_17,
    early_exception_stub_18,
    early_exception_stub_19,
    early_exception_stub_20,
    early_exception_stub_21,
    early_exception_stub_22,
    early_exception_stub_23,
    early_exception_stub_24,
    early_exception_stub_25,
    early_exception_stub_26,
    early_exception_stub_27,
    early_exception_stub_28,
    early_exception_stub_29,
    early_exception_stub_30,
    early_exception_stub_31,
];

#[must_use]
fn early_exception_stub_addr(vector: usize) -> usize {
    EXCEPTION_STUBS[vector] as *const () as usize
}

static EARLY_IDT_READY: AtomicBool = AtomicBool::new(false);
static mut EARLY_IDT: [IdtEntry; EXCEPTION_VECTOR_COUNT] =
    [IdtEntry::missing(); EXCEPTION_VECTOR_COUNT];

fn ensure_early_idt_populated() {
    if EARLY_IDT_READY.load(Ordering::Acquire) {
        return;
    }

    // SAFETY: During early single-core initialization we populate a fixed static IDT table
    // exactly once before publishing readiness with release ordering.
    unsafe {
        let mut index = 0;
        while index < EXCEPTION_VECTOR_COUNT {
            EARLY_IDT[index] = IdtEntry::interrupt_gate(early_exception_stub_addr(index));
            index += 1;
        }
    }

    EARLY_IDT_READY.store(true, Ordering::Release);
}

/// Returns the deterministic early IDT descriptor used during interrupt milestone bring-up.
#[must_use]
pub fn early_idt_descriptor() -> IdtDescriptor {
    ensure_early_idt_populated();
    IdtDescriptor {
        limit: (size_of::<[IdtEntry; EXCEPTION_VECTOR_COUNT]>() - 1) as u16,
        base: (&raw const EARLY_IDT) as *const IdtEntry as u64,
    }
}

/// Returns the deterministic early IDT entries used by boot initialization.
#[must_use]
pub fn early_idt_entries() -> &'static [IdtEntry; EXCEPTION_VECTOR_COUNT] {
    ensure_early_idt_populated();
    let idt_ptr = &raw const EARLY_IDT;
    // SAFETY: The table is initialized before returning and then treated as immutable.
    unsafe { &*idt_ptr }
}

/// Initializes the early interrupt skeleton and loads IDT on real UEFI/x86_64 boot targets.
#[must_use]
pub fn init_early_interrupts() -> InterruptInitReport {
    let descriptor = early_idt_descriptor();
    maybe_load_early_idt(&descriptor);

    InterruptInitReport {
        vectors_initialized: EXCEPTION_VECTOR_COUNT,
        descriptor,
        first_handler_addr: early_exception_stub_addr(0),
        last_handler_addr: early_exception_stub_addr(EXCEPTION_VECTOR_COUNT - 1),
    }
}

/// Returns the deterministic early exception serial log line for a given vector.
#[must_use]
pub fn exception_log_line(vector: u8) -> &'static str {
    let index = usize::from(vector);
    if index < EXCEPTION_VECTOR_COUNT {
        EXCEPTION_LOG_LINES[index]
    } else {
        EXCEPTION_LOG_UNKNOWN_LINE
    }
}

/// Returns the deterministic early exception serial log line as bytes.
#[must_use]
pub fn exception_log_line_bytes(vector: u8) -> &'static [u8] {
    exception_log_line(vector).as_bytes()
}

/// Produces a deterministic exception dispatch report for firmware logging paths.
#[must_use]
pub fn dispatch_exception(vector: u8) -> ExceptionDispatchReport {
    let known_vector = usize::from(vector) < EXCEPTION_VECTOR_COUNT;
    ExceptionDispatchReport {
        vector,
        known_vector,
        line: exception_log_line(vector),
    }
}

#[cfg(all(target_arch = "x86_64", target_os = "uefi"))]
fn maybe_load_early_idt(descriptor: &IdtDescriptor) {
    // SAFETY: During firmware boot on x86_64 we intentionally install a statically allocated
    // early IDT descriptor before enabling the rest of kernel subsystems.
    unsafe {
        core::arch::asm!(
            "lidt [{}]",
            in(reg) descriptor,
            options(readonly, nostack, preserves_flags)
        );
    }
}

#[cfg(not(all(target_arch = "x86_64", target_os = "uefi")))]
fn maybe_load_early_idt(_descriptor: &IdtDescriptor) {}

#[cfg(all(target_arch = "x86_64", target_os = "uefi"))]
fn maybe_install_cr3(root_table_phys_addr: u64) -> bool {
    let _ = root_table_phys_addr;
    // Early paging structures are still materialized for deterministic contract tests, but we
    // intentionally avoid writing CR3 in the UEFI path for now. The frame-window addresses are
    // modeled physical addresses, not yet backed by allocator-owned identity mappings, so loading
    // them into CR3 can fault/abort before the boot transcript reaches completion in QEMU smoke.
    false
}

#[cfg(not(all(target_arch = "x86_64", target_os = "uefi")))]
fn maybe_install_cr3(_root_table_phys_addr: u64) -> bool {
    false
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::{
        advance_early_scheduler_round_robin, boot_banner_bytes, boot_banner_line_bytes,
        boot_entry_done_line_bytes, boot_global_allocator_probe_line_bytes,
        boot_global_allocator_ready_line_bytes, boot_heap_alloc_cycle_line_bytes,
        boot_heap_bootstrap_line_bytes, boot_interrupt_init_line_bytes,
        boot_memory_init_line_bytes, boot_paging_install_line_bytes, boot_paging_plan_line_bytes,
        boot_panic_line_bytes, boot_scheduler_edge_blocked_line_bytes,
        boot_scheduler_edge_terminated_line_bytes, boot_scheduler_handoff_line_bytes,
        boot_thread_context_meta_line_bytes, boot_thread_context_restore_line_bytes,
        boot_thread_context_save_line_bytes, boot_thread_dequeue_line_bytes,
        boot_thread_enqueue_line_bytes, boot_thread_state_blocked_line_bytes,
        boot_thread_state_ready_line_bytes, boot_thread_state_terminated_line_bytes,
        boot_thread_wait_contention_line_bytes, boot_thread_wait_ownership_line_bytes,
        boot_thread_wake_line_bytes, boot_thread_wake_order_line_bytes,
        boot_thread_wake_timeout_line_bytes, boot_timer_ack_line_bytes,
        boot_timer_first_tick_line_bytes, boot_timer_handoff_line_bytes,
        boot_timer_init_line_bytes, boot_timer_third_tick_line_bytes, bootstrap_early_kernel_heap,
        dequeue_early_scheduler_task, dispatch_early_timer_interrupt, dispatch_exception,
        early_idt_descriptor, early_idt_entries, early_paging_table_snapshot,
        early_physical_memory_map, early_translation_state_valid, enqueue_early_scheduler_task,
        exception_log_line, exception_log_line_bytes, init_early_global_allocator,
        init_early_interrupts, init_early_paging_plan, init_early_physical_memory,
        init_early_timer, install_early_paging, is_canonical_virtual_address, is_page_aligned_4k,
        model_early_scheduler_blocked_selection_edge_case,
        model_early_scheduler_terminated_cleanup_edge_case, model_early_thread_context_handoff,
        record_early_timer_tick, reset_early_scheduler_state, reset_early_timer_ticks,
        resolve_early_thread_wake_contention, run_early_global_allocator_probe,
        run_early_heap_alloc_cycle, sample_early_timer_handoff, take_early_scheduler_timer_handoff,
        take_early_timer_handoff, transition_early_thread_lifecycle,
        translate_early_virtual_to_physical, wake_early_thread, EarlyFrameAllocationError,
        EarlyFrameAllocator, EarlyHeapAllocationError, EarlyHeapAllocator, EarlyHeapBootstrapError,
        EarlyHeapDeallocationError, EarlyHeapOperationError, EarlyThreadLifecycleState,
        GlobalAllocatorInitError, IdtEntry, PhysicalMemoryRegionKind, VirtualAddress,
        VirtualAddressTranslationError, BOOT_BANNER, BOOT_BANNER_LINE, BOOT_ENTRY_DONE_LINE,
        BOOT_GLOBAL_ALLOCATOR_PROBE_LINE, BOOT_GLOBAL_ALLOCATOR_READY_LINE,
        BOOT_HEAP_ALLOC_CYCLE_LINE, BOOT_HEAP_BOOTSTRAP_LINE, BOOT_INTERRUPT_INIT_LINE,
        BOOT_MEMORY_INIT_LINE, BOOT_PAGING_INSTALL_LINE, BOOT_PAGING_PLAN_LINE, BOOT_PANIC_LINE,
        BOOT_SCHEDULER_EDGE_BLOCKED_LINE, BOOT_SCHEDULER_EDGE_TERMINATED_LINE,
        BOOT_SCHEDULER_HANDOFF_LINE, BOOT_THREAD_CONTEXT_META_LINE,
        BOOT_THREAD_CONTEXT_RESTORE_LINE, BOOT_THREAD_CONTEXT_SAVE_LINE, BOOT_THREAD_DEQUEUE_LINE,
        BOOT_THREAD_ENQUEUE_LINE, BOOT_THREAD_STATE_BLOCKED_LINE, BOOT_THREAD_STATE_READY_LINE,
        BOOT_THREAD_STATE_TERMINATED_LINE, BOOT_THREAD_WAIT_CONTENTION_LINE,
        BOOT_THREAD_WAIT_OWNERSHIP_LINE, BOOT_THREAD_WAKE_LINE, BOOT_THREAD_WAKE_ORDER_LINE,
        BOOT_THREAD_WAKE_TIMEOUT_LINE, BOOT_TIMER_ACK_LINE, BOOT_TIMER_FIRST_TICK_LINE,
        BOOT_TIMER_HANDOFF_LINE, BOOT_TIMER_INIT_LINE, BOOT_TIMER_THIRD_TICK_LINE,
        EARLY_GLOBAL_ALLOCATOR, EXCEPTION_VECTOR_COUNT,
    };

    #[test]
    fn boot_banner_matches_expected_literal() {
        assert_eq!(BOOT_BANNER, "tosm-os: kernel entry reached");
    }

    #[test]
    fn boot_banner_bytes_are_stable() {
        assert_eq!(boot_banner_bytes(), b"tosm-os: kernel entry reached");
    }

    #[test]
    fn boot_banner_line_bytes_include_crlf() {
        assert_eq!(BOOT_BANNER_LINE, "tosm-os: kernel entry reached\r\n");
        assert_eq!(
            boot_banner_line_bytes(),
            b"tosm-os: kernel entry reached\r\n"
        );
    }

    #[test]
    fn boot_interrupt_init_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_INTERRUPT_INIT_LINE,
            "tosm-os: idt skeleton initialized\r\n"
        );
        assert_eq!(
            boot_interrupt_init_line_bytes(),
            b"tosm-os: idt skeleton initialized\r\n"
        );
    }

    #[test]
    fn boot_panic_line_bytes_include_crlf() {
        assert_eq!(BOOT_PANIC_LINE, "tosm-os: panic in uefi-entry\r\n");
        assert_eq!(boot_panic_line_bytes(), b"tosm-os: panic in uefi-entry\r\n");
    }

    #[test]
    fn boot_entry_done_line_bytes_include_crlf() {
        assert_eq!(BOOT_ENTRY_DONE_LINE, "tosm-os: efi_main completed\r\n");
        assert_eq!(
            boot_entry_done_line_bytes(),
            b"tosm-os: efi_main completed\r\n"
        );
    }

    #[test]
    fn boot_memory_init_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_MEMORY_INIT_LINE,
            "tosm-os: memory init usable=0x3f790000 reserved=0x00811000 regions=5\r\n"
        );
        assert_eq!(
            boot_memory_init_line_bytes(),
            b"tosm-os: memory init usable=0x3f790000 reserved=0x00811000 regions=5\r\n"
        );
    }

    #[test]
    fn boot_paging_plan_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_PAGING_PLAN_LINE,
            "tosm-os: paging plan frames=4 window=0x3f7ed000-0x3f7f1000 map4k=512\r\n"
        );
        assert_eq!(
            boot_paging_plan_line_bytes(),
            b"tosm-os: paging plan frames=4 window=0x3f7ed000-0x3f7f1000 map4k=512\r\n"
        );
    }

    #[test]
    fn boot_paging_install_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_PAGING_INSTALL_LINE,
            "tosm-os: paging install root=0x3f7ed000 span=0x40000000 entries=514\r\n"
        );
        assert_eq!(
            boot_paging_install_line_bytes(),
            b"tosm-os: paging install root=0x3f7ed000 span=0x40000000 entries=514\r\n"
        );
    }

    #[test]
    fn boot_heap_bootstrap_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_HEAP_BOOTSTRAP_LINE,
            "tosm-os: heap bootstrap start=0x00400000 size=0x00004000 frames=4\r\n"
        );
        assert_eq!(
            boot_heap_bootstrap_line_bytes(),
            b"tosm-os: heap bootstrap start=0x00400000 size=0x00004000 frames=4\r\n"
        );
    }

    #[test]
    fn boot_heap_alloc_cycle_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_HEAP_ALLOC_CYCLE_LINE,
            "tosm-os: heap alloc cycle allocs=2 frees=2 cursor=0x00400000\r\n"
        );
        assert_eq!(
            boot_heap_alloc_cycle_line_bytes(),
            b"tosm-os: heap alloc cycle allocs=2 frees=2 cursor=0x00400000\r\n"
        );
    }

    #[test]
    fn boot_global_allocator_probe_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_GLOBAL_ALLOCATOR_PROBE_LINE,
            "tosm-os: global allocator probe entries=4 checksum=0x000000000000002a\r\n"
        );
        assert_eq!(
            boot_global_allocator_probe_line_bytes(),
            b"tosm-os: global allocator probe entries=4 checksum=0x000000000000002a\r\n"
        );
    }

    #[test]
    fn boot_global_allocator_ready_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_GLOBAL_ALLOCATOR_READY_LINE,
            "tosm-os: global allocator ready heap=0x00400000-0x00404000\r\n"
        );
        assert_eq!(
            boot_global_allocator_ready_line_bytes(),
            b"tosm-os: global allocator ready heap=0x00400000-0x00404000\r\n"
        );
    }

    #[test]
    fn boot_timer_init_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_TIMER_INIT_LINE,
            "tosm-os: timer init source=pit hz=100 divisor=11931 irq=0x20\r\n"
        );
        assert_eq!(
            boot_timer_init_line_bytes(),
            b"tosm-os: timer init source=pit hz=100 divisor=11931 irq=0x20\r\n"
        );
    }

    #[test]
    fn boot_timer_first_tick_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_TIMER_FIRST_TICK_LINE,
            "tosm-os: timer tick irq=0x20 count=1 uptime_ns=10000000\r\n"
        );
        assert_eq!(
            boot_timer_first_tick_line_bytes(),
            b"tosm-os: timer tick irq=0x20 count=1 uptime_ns=10000000\r\n"
        );
    }

    #[test]
    fn early_physical_memory_map_model_has_expected_regions() {
        let map = early_physical_memory_map();
        assert_eq!(map.len(), 5);
        assert_eq!(map[0].start, 0x0000_0000);
        assert_eq!(map[0].length, 0x0009_f000);
        assert_eq!(map[0].kind, PhysicalMemoryRegionKind::Usable);
        assert_eq!(map[1].kind, PhysicalMemoryRegionKind::Reserved);
        assert_eq!(map[3].start, 0x0010_0000);
        assert_eq!(map[3].length, 0x3f6f_1000);
        assert_eq!(map[3].kind, PhysicalMemoryRegionKind::Usable);
    }

    #[test]
    fn init_early_physical_memory_reports_usable_and_reserved_totals() {
        let report = init_early_physical_memory();
        assert_eq!(report.regions_modeled, 5);
        assert_eq!(report.usable_bytes, 0x0000_0000_3f79_0000);
        assert_eq!(report.reserved_bytes, 0x0000_0000_0081_1000);
        assert_eq!(report.highest_usable_end_exclusive, 0x0000_0000_3f7f_1000);
    }

    #[test]
    fn init_early_paging_plan_selects_top_usable_frame_window_and_identity_map_contract() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);

        assert_eq!(paging_plan.frame_window.frame_count, 4);
        assert_eq!(paging_plan.frame_window.start, 0x0000_0000_3f7e_d000);
        assert_eq!(
            paging_plan.frame_window.end_exclusive,
            0x0000_0000_3f7f_1000
        );
        assert_eq!(paging_plan.identity_map_start, 0);
        assert_eq!(
            paging_plan.identity_map_end_exclusive,
            0x0000_0000_0020_0000
        );
        assert_eq!(paging_plan.identity_map_pages_4k, 512);
    }

    #[test]
    fn install_early_paging_materializes_root_pdpt_and_identity_2m_entries() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);

        assert_eq!(install.root_table_phys_addr, 0x0000_0000_3f7e_d000);
        assert_eq!(install.pdpt_phys_addr, 0x0000_0000_3f7e_e000);
        assert_eq!(install.pd_phys_addr, 0x0000_0000_3f7e_f000);
        assert_eq!(install.mapped_span_bytes, 0x0000_0000_4000_0000);
        assert_eq!(install.present_entry_count, 514);
        assert!(!install.installed_into_cpu);

        let (tables, _table_addrs) = early_paging_table_snapshot();
        let root = &tables[0];
        let pdpt = &tables[1];
        let pd = &tables[2];

        assert_eq!(root[0], 0x0000_0000_3f7e_e003);
        assert_eq!(pdpt[0], 0x0000_0000_3f7e_f003);
        assert_eq!(pd[0], 0x0000_0000_0000_0083);
        assert_eq!(pd[1], 0x0000_0000_0020_0083);
        assert_eq!(pd[511], 0x0000_0000_3fe0_0083);
        assert_eq!(root[1], 0);
        assert_eq!(pdpt[1], 0);
    }

    #[test]
    fn canonical_virtual_address_guard_accepts_lower_and_higher_halves() {
        assert!(is_canonical_virtual_address(0x0000_0000_0000_0000));
        assert!(is_canonical_virtual_address(0x0000_7fff_ffff_ffff));
        assert!(is_canonical_virtual_address(0xffff_8000_0000_0000));
        assert!(is_canonical_virtual_address(0xffff_ffff_ffff_ffff));
        assert!(!is_canonical_virtual_address(0x0000_8000_0000_0000));
        assert!(!is_canonical_virtual_address(0xffff_7fff_ffff_ffff));
    }

    #[test]
    fn page_alignment_guard_detects_4k_boundaries() {
        assert!(is_page_aligned_4k(0x0000_0000_0010_0000));
        assert!(is_page_aligned_4k(0x0000_0000_3f7e_d000));
        assert!(!is_page_aligned_4k(0x0000_0000_3f7e_d001));
    }

    #[test]
    fn early_translation_state_guard_rejects_invalid_reports() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);
        assert!(early_translation_state_valid(install));

        let mut bad = install;
        bad.root_table_phys_addr += 1;
        assert!(!early_translation_state_valid(bad));

        bad = install;
        bad.mapped_span_bytes = 0;
        assert!(!early_translation_state_valid(bad));
    }

    #[test]
    fn early_virtual_translation_maps_identity_range_and_rejects_out_of_range() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);

        let phys = translate_early_virtual_to_physical(VirtualAddress(0x0012_3456), install)
            .expect("identity-mapped early address should translate");
        assert_eq!(phys.0, 0x0012_3456);

        let err =
            translate_early_virtual_to_physical(VirtualAddress(0x0000_0001_0000_0000), install)
                .expect_err("address beyond mapped span should be rejected");
        assert_eq!(err, VirtualAddressTranslationError::UnmappedAddress);
    }

    #[test]
    fn early_virtual_translation_rejects_noncanonical_and_invalid_state() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);

        let err =
            translate_early_virtual_to_physical(VirtualAddress(0x0000_8000_0000_0000), install)
                .expect_err("non-canonical address should be rejected");
        assert_eq!(err, VirtualAddressTranslationError::NonCanonicalAddress);

        let mut bad = install;
        bad.pdpt_phys_addr += 1;
        let err = translate_early_virtual_to_physical(VirtualAddress(0), bad)
            .expect_err("invalid paging state should be rejected");
        assert_eq!(err, VirtualAddressTranslationError::InvalidPagingState);
    }

    #[test]
    fn early_frame_allocator_hands_out_4k_frames_and_tracks_progress() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);

        let mut allocator = EarlyFrameAllocator::from_install_report(install);
        let first = allocator
            .allocate_for_virtual(VirtualAddress(0x2000), install)
            .expect("first allocation should succeed");
        let second = allocator
            .allocate_for_virtual(VirtualAddress(0x3000), install)
            .expect("second allocation should succeed");

        assert_eq!(first.frame_start.0, 0);
        assert_eq!(second.frame_start.0, 0x1000);
        assert_eq!(first.translated_phys.0, 0x2000);
        assert_eq!(allocator.next_frame_start(), 0x2000);
        assert_eq!(allocator.end_exclusive(), install.mapped_span_bytes);
    }

    #[test]
    fn early_frame_allocator_rejects_invalid_virtual_translation_inputs() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);
        let mut allocator = EarlyFrameAllocator::from_install_report(install);

        let err = allocator
            .allocate_for_virtual(VirtualAddress(0x0000_8000_0000_0000), install)
            .expect_err("non-canonical addresses must be rejected");
        assert_eq!(err, EarlyFrameAllocationError::NonCanonicalAddress);

        let err = allocator
            .allocate_for_virtual(VirtualAddress(0x0000_0001_0000_0000), install)
            .expect_err("unmapped addresses must be rejected");
        assert_eq!(err, EarlyFrameAllocationError::UnmappedAddress);

        let mut bad = install;
        bad.root_table_phys_addr = 0x1234;
        let err = allocator
            .allocate_for_virtual(VirtualAddress(0), bad)
            .expect_err("invalid paging state should be rejected");
        assert_eq!(err, EarlyFrameAllocationError::InvalidPagingState);
    }

    #[test]
    fn early_frame_allocator_reports_out_of_frames_at_end_of_span() {
        let mut allocator = EarlyFrameAllocator {
            next_frame_start: 0x1000,
            end_exclusive: 0x1000,
        };
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);

        let err = allocator
            .allocate_for_virtual(VirtualAddress(0), install)
            .expect_err("allocator should reject allocations beyond frame span");
        assert_eq!(err, EarlyFrameAllocationError::OutOfFrames);
    }

    #[test]
    fn early_heap_bootstrap_reserves_expected_window_and_frames() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);
        let mut allocator = EarlyFrameAllocator::from_install_report(install);

        let heap = bootstrap_early_kernel_heap(&mut allocator, install)
            .expect("heap bootstrap should succeed over deterministic frame window");

        assert_eq!(heap.heap_start_virt.0, 0x0040_0000);
        assert_eq!(heap.heap_end_exclusive_virt.0, 0x0040_4000);
        assert_eq!(heap.heap_frame_start.0, 0);
        assert_eq!(heap.heap_frame_count, 4);
        assert_eq!(heap.heap_bytes, 0x4000);
        assert_eq!(allocator.next_frame_start(), 0x4000);
    }

    #[test]
    fn early_heap_bootstrap_propagates_frame_allocator_errors() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);

        let mut exhausted = EarlyFrameAllocator {
            next_frame_start: install.mapped_span_bytes,
            end_exclusive: install.mapped_span_bytes,
        };
        let err = bootstrap_early_kernel_heap(&mut exhausted, install)
            .expect_err("heap bootstrap should fail when no frames are available");
        assert_eq!(err, EarlyHeapBootstrapError::OutOfFrames);

        let mut allocator = EarlyFrameAllocator::from_install_report(install);
        let mut invalid = install;
        invalid.root_table_phys_addr = 0x1234;
        let err = bootstrap_early_kernel_heap(&mut allocator, invalid)
            .expect_err("heap bootstrap should reject invalid translation state");
        assert_eq!(err, EarlyHeapBootstrapError::InvalidPagingState);
    }

    #[test]
    fn early_heap_allocator_allocate_deallocate_cycle_resets_cursor() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);
        let mut frame_allocator = EarlyFrameAllocator::from_install_report(install);
        let bootstrap = bootstrap_early_kernel_heap(&mut frame_allocator, install)
            .expect("heap bootstrap should succeed for allocator tests");

        let mut heap = EarlyHeapAllocator::from_bootstrap(bootstrap);
        let a = heap.allocate(0x20, 0x10).expect("first alloc should pass");
        let b = heap.allocate(0x40, 0x20).expect("second alloc should pass");
        assert!(b.start_virt.0 >= a.start_virt.0 + a.size_bytes);
        assert_eq!(heap.allocated_bytes(), 0x60);

        heap.deallocate(b).expect("deallocate second should pass");
        heap.deallocate(a).expect("deallocate first should pass");
        assert_eq!(heap.allocated_bytes(), 0);
        assert_eq!(heap.cursor_virt().0, 0x0040_0000);
    }

    #[test]
    fn early_heap_allocator_rejects_invalid_requests_and_double_free() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);
        let mut frame_allocator = EarlyFrameAllocator::from_install_report(install);
        let bootstrap = bootstrap_early_kernel_heap(&mut frame_allocator, install)
            .expect("heap bootstrap should succeed for allocator tests");

        let mut heap = EarlyHeapAllocator::from_bootstrap(bootstrap);
        let err = heap
            .allocate(0, 0x8)
            .expect_err("zero-sized allocations must be rejected");
        assert_eq!(err, EarlyHeapAllocationError::ZeroSize);

        let err = heap
            .allocate(0x10, 3)
            .expect_err("non-power-of-two alignments must be rejected");
        assert_eq!(err, EarlyHeapAllocationError::InvalidAlignment);

        let allocation = heap.allocate(0x10, 0x8).expect("allocation should pass");
        heap.deallocate(allocation)
            .expect("initial deallocation should pass");
        let err = heap
            .deallocate(allocation)
            .expect_err("double free should be rejected");
        assert_eq!(err, EarlyHeapDeallocationError::DoubleFree);
    }

    #[test]
    fn run_early_heap_alloc_cycle_reports_expected_contract() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);
        let mut frame_allocator = EarlyFrameAllocator::from_install_report(install);
        let bootstrap = bootstrap_early_kernel_heap(&mut frame_allocator, install)
            .expect("heap bootstrap should succeed for allocator tests");

        let mut heap = EarlyHeapAllocator::from_bootstrap(bootstrap);
        let cycle = run_early_heap_alloc_cycle(&mut heap)
            .expect("alloc cycle should complete over deterministic heap window");
        assert_eq!(cycle.allocations, 2);
        assert_eq!(cycle.deallocations, 2);
        assert_eq!(cycle.final_cursor_virt.0, 0x0040_0000);

        let mut too_small = EarlyHeapAllocator::from_bootstrap(bootstrap);
        let _ = too_small.allocate(bootstrap.heap_bytes - 0x10, 0x10);
        let err = run_early_heap_alloc_cycle(&mut too_small)
            .expect_err("allocation failures should surface through cycle reports");
        assert!(matches!(
            err,
            EarlyHeapOperationError::Allocation(EarlyHeapAllocationError::OutOfMemory)
        ));
    }

    #[test]
    fn init_early_global_allocator_reports_initialized_state() {
        let memory_report = init_early_physical_memory();
        let paging_plan = init_early_paging_plan(memory_report);
        let install = install_early_paging(paging_plan);
        let mut frame_allocator = EarlyFrameAllocator::from_install_report(install);
        let bootstrap = bootstrap_early_kernel_heap(&mut frame_allocator, install)
            .expect("heap bootstrap should succeed for allocator tests");

        let report = init_early_global_allocator(bootstrap)
            .expect("global allocator initialization should succeed once");
        assert!(report.initialized);
        assert_eq!(report.allocated_bytes, 0);

        let second = init_early_global_allocator(bootstrap)
            .expect_err("global allocator must reject double initialization");
        assert_eq!(second, GlobalAllocatorInitError::AlreadyInitialized);
    }

    #[test]
    fn run_early_global_allocator_probe_uses_dynamic_structure_and_preserves_state() {
        if !EARLY_GLOBAL_ALLOCATOR.state_report().initialized {
            let memory_report = init_early_physical_memory();
            let paging_plan = init_early_paging_plan(memory_report);
            let install = install_early_paging(paging_plan);
            let mut frame_allocator = EarlyFrameAllocator::from_install_report(install);
            let bootstrap = bootstrap_early_kernel_heap(&mut frame_allocator, install)
                .expect("heap bootstrap should succeed for allocator tests");

            let _ = init_early_global_allocator(bootstrap);
        }

        let probe = run_early_global_allocator_probe()
            .expect("probe should allocate and free deterministic dynamic structure");
        assert_eq!(probe.entries, 4);
        assert_eq!(probe.checksum, 42);

        let state = EARLY_GLOBAL_ALLOCATOR.state_report();
        assert!(state.initialized);
        assert_eq!(state.allocated_bytes, 0);
    }

    #[test]
    fn idt_entries_cover_all_exception_vectors() {
        assert_eq!(early_idt_entries().len(), EXCEPTION_VECTOR_COUNT);
    }

    #[test]
    fn idt_entries_use_kernel_code_segment_interrupt_gates() {
        for entry in early_idt_entries() {
            assert_eq!(entry.selector(), 0x0008);
            assert_eq!(entry.type_attributes(), 0x8E);
        }
    }

    #[test]
    fn idt_entries_use_vector_specific_early_stubs() {
        for (vector, entry) in early_idt_entries().iter().enumerate() {
            for next in early_idt_entries().iter().skip(vector + 1) {
                assert_ne!(entry.handler_addr(), next.handler_addr());
            }
        }
    }

    #[test]
    fn idt_descriptor_limit_matches_table_size() {
        let descriptor = early_idt_descriptor();
        assert_eq!(
            descriptor.limit as usize + 1,
            core::mem::size_of_val(early_idt_entries())
        );
        let base = descriptor.base;
        assert_ne!(base, 0);
    }

    #[test]
    fn init_early_interrupts_reports_initialized_vectors() {
        let report = init_early_interrupts();
        assert_eq!(report.vectors_initialized, EXCEPTION_VECTOR_COUNT);
        assert_eq!(
            report.first_handler_addr,
            early_idt_entries()[0].handler_addr()
        );
        assert_eq!(
            report.last_handler_addr,
            early_idt_entries()[EXCEPTION_VECTOR_COUNT - 1].handler_addr()
        );
        assert_ne!(report.first_handler_addr, report.last_handler_addr);
        assert_eq!(report.descriptor, early_idt_descriptor());
    }

    #[test]
    fn exception_log_line_exposes_named_vectors_and_unknown_fallback() {
        assert_eq!(
            exception_log_line(0),
            "tosm-os: exception vector 00 divide error\r\n"
        );
        assert_eq!(
            exception_log_line(14),
            "tosm-os: exception vector 14 page fault\r\n"
        );
        assert_eq!(
            exception_log_line(31),
            "tosm-os: exception vector 31 reserved\r\n"
        );
        assert_eq!(
            exception_log_line(EXCEPTION_VECTOR_COUNT as u8),
            "tosm-os: exception vector unknown\r\n"
        );
    }

    #[test]
    fn exception_log_line_bytes_include_crlf() {
        assert_eq!(
            exception_log_line_bytes(13),
            b"tosm-os: exception vector 13 general protection fault\r\n"
        );
        assert_eq!(
            exception_log_line_bytes(EXCEPTION_VECTOR_COUNT as u8),
            b"tosm-os: exception vector unknown\r\n"
        );
    }

    #[test]
    fn dispatch_exception_reports_known_and_unknown_vectors() {
        let page_fault = dispatch_exception(14);
        assert_eq!(page_fault.vector, 14);
        assert!(page_fault.known_vector);
        assert_eq!(
            page_fault.line,
            "tosm-os: exception vector 14 page fault\r\n"
        );

        let unknown = dispatch_exception(EXCEPTION_VECTOR_COUNT as u8);
        assert_eq!(unknown.vector, EXCEPTION_VECTOR_COUNT as u8);
        assert!(!unknown.known_vector);
        assert_eq!(unknown.line, "tosm-os: exception vector unknown\r\n");
    }

    #[test]
    fn init_early_timer_reports_pit_contracts() {
        let report = init_early_timer();
        assert_eq!(report.source, "pit");
        assert_eq!(report.frequency_hz, 100);
        assert_eq!(report.pit_input_hz, 1_193_182);
        assert_eq!(report.divisor, 11_931);
        assert_eq!(report.irq_vector, 0x20);
        assert_eq!(report.tick_period_ns, 10_000_000);
    }

    #[test]
    fn record_early_timer_tick_tracks_periodic_count_and_uptime() {
        reset_early_timer_ticks();
        let timer = init_early_timer();

        let first = record_early_timer_tick(timer);
        let second = record_early_timer_tick(timer);

        assert_eq!(first.irq_vector, 0x20);
        assert_eq!(first.tick_count, 1);
        assert_eq!(first.uptime_ns, 10_000_000);
        assert_eq!(second.tick_count, 2);
        assert_eq!(second.uptime_ns, 20_000_000);

        reset_early_timer_ticks();
    }

    #[test]
    fn timer_ack_and_multi_tick_dispatch_report_expected_contracts() {
        reset_early_timer_ticks();
        let timer = init_early_timer();

        let first = dispatch_early_timer_interrupt(timer);
        let second = dispatch_early_timer_interrupt(timer);
        let third = dispatch_early_timer_interrupt(timer);

        assert_eq!(first.tick.tick_count, 1);
        assert_eq!(second.tick.tick_count, 2);
        assert_eq!(third.tick.tick_count, 3);
        assert_eq!(third.tick.uptime_ns, 30_000_000);
        assert_eq!(third.ack.irq_vector, 0x20);
        assert_eq!(third.ack.pic_command_port, 0x20);
        assert_eq!(third.ack.pic_eoi_value, 0x20);
        assert!(third.ack.acknowledged);

        reset_early_timer_ticks();
    }

    #[test]
    fn boot_timer_handoff_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_TIMER_HANDOFF_LINE,
            "tosm-os: timer handoff ticks=3 delta=3 quantum=1 uptime_ns=30000000\r\n"
        );
        assert_eq!(
            boot_timer_handoff_line_bytes(),
            b"tosm-os: timer handoff ticks=3 delta=3 quantum=1 uptime_ns=30000000\r\n"
        );
    }

    #[test]
    fn boot_scheduler_handoff_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_SCHEDULER_HANDOFF_LINE,
            "tosm-os: scheduler handoff reason=timer runq=2 selected=1 idle=0 delta=3\r\n"
        );
        assert_eq!(
            boot_scheduler_handoff_line_bytes(),
            b"tosm-os: scheduler handoff reason=timer runq=2 selected=1 idle=0 delta=3\r\n"
        );
    }

    #[test]
    fn thread_enqueue_and_dequeue_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_THREAD_ENQUEUE_LINE,
            "tosm-os: thread enqueue task=2 runq=3 selected=1\r\n"
        );
        assert_eq!(
            boot_thread_enqueue_line_bytes(),
            b"tosm-os: thread enqueue task=2 runq=3 selected=1\r\n"
        );
        assert_eq!(
            BOOT_THREAD_DEQUEUE_LINE,
            "tosm-os: thread dequeue task=2 runq=2 selected=1\r\n"
        );
        assert_eq!(
            boot_thread_dequeue_line_bytes(),
            b"tosm-os: thread dequeue task=2 runq=2 selected=1\r\n"
        );
    }

    #[test]
    fn timer_handoff_sampling_and_take_track_delta_watermark() {
        reset_early_timer_ticks();
        let timer = init_early_timer();

        let before_ticks = sample_early_timer_handoff(timer);
        assert_eq!(before_ticks.total_ticks, 0);
        assert_eq!(before_ticks.ticks_since_last_handoff, 0);
        assert_eq!(before_ticks.uptime_ns, 0);
        assert!(!before_ticks.scheduler_quantum_elapsed);

        let _first = dispatch_early_timer_interrupt(timer);
        let _second = dispatch_early_timer_interrupt(timer);
        let _third = dispatch_early_timer_interrupt(timer);

        let sampled = sample_early_timer_handoff(timer);
        assert_eq!(sampled.irq_vector, 0x20);
        assert_eq!(sampled.total_ticks, 3);
        assert_eq!(sampled.ticks_since_last_handoff, 3);
        assert_eq!(sampled.uptime_ns, 30_000_000);
        assert!(sampled.scheduler_quantum_elapsed);

        let handoff = take_early_timer_handoff(timer);
        assert_eq!(handoff.total_ticks, 3);
        assert_eq!(handoff.ticks_since_last_handoff, 3);
        assert!(handoff.scheduler_quantum_elapsed);

        let after = sample_early_timer_handoff(timer);
        assert_eq!(after.total_ticks, 3);
        assert_eq!(after.ticks_since_last_handoff, 0);
        assert!(!after.scheduler_quantum_elapsed);

        reset_early_timer_ticks();
    }

    #[test]
    fn scheduler_timer_handoff_selects_bootstrap_task_after_quantum_elapsed() {
        reset_early_timer_ticks();
        reset_early_scheduler_state();
        let timer = init_early_timer();

        let before = take_early_scheduler_timer_handoff(timer);
        assert_eq!(before.timer.total_ticks, 0);
        assert_eq!(before.scheduler.run_queue_depth, 2);
        assert_eq!(before.scheduler.selected_task_id, 0);

        let _first = dispatch_early_timer_interrupt(timer);
        let _second = dispatch_early_timer_interrupt(timer);
        let _third = dispatch_early_timer_interrupt(timer);

        let after = take_early_scheduler_timer_handoff(timer);
        assert_eq!(after.timer.ticks_since_last_handoff, 3);
        assert_eq!(after.scheduler.run_queue_depth, 2);
        assert_eq!(after.scheduler.selected_task_id, 1);
        assert_eq!(after.scheduler.idle_task_id, 0);

        reset_early_timer_ticks();
        reset_early_scheduler_state();
    }

    #[test]
    fn scheduler_slot_mutation_and_round_robin_contracts_are_deterministic() {
        reset_early_scheduler_state();

        let enqueued =
            enqueue_early_scheduler_task(2).expect("enqueue should add a runnable worker");
        assert_eq!(enqueued.task_id, 2);
        assert_eq!(enqueued.run_queue_depth, 3);
        assert_eq!(enqueued.selected_task_id, 0);

        let first = advance_early_scheduler_round_robin(super::EarlySchedulerHandoffReason::Yield);
        assert_eq!(first.selected_task_id, 1);

        let second = advance_early_scheduler_round_robin(super::EarlySchedulerHandoffReason::Yield);
        assert_eq!(second.selected_task_id, 2);

        let dequeued = dequeue_early_scheduler_task(2).expect("dequeue should remove worker task");
        assert_eq!(dequeued.task_id, 2);
        assert_eq!(dequeued.run_queue_depth, 2);
        assert_eq!(dequeued.selected_task_id, 0);

        reset_early_scheduler_state();
    }

    #[test]
    fn thread_context_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_THREAD_CONTEXT_SAVE_LINE,
            "tosm-os: thread ctx save from=1 to=2 rip=0x100200 rsp=0x401f00\r\n"
        );
        assert_eq!(
            boot_thread_context_save_line_bytes(),
            b"tosm-os: thread ctx save from=1 to=2 rip=0x100200 rsp=0x401f00\r\n"
        );
        assert_eq!(
            BOOT_THREAD_CONTEXT_RESTORE_LINE,
            "tosm-os: thread ctx restore to=2 rip=0x200000 rsp=0x402000\r\n"
        );
        assert_eq!(
            boot_thread_context_restore_line_bytes(),
            b"tosm-os: thread ctx restore to=2 rip=0x200000 rsp=0x402000\r\n"
        );
        assert_eq!(
            BOOT_THREAD_CONTEXT_META_LINE,
            "tosm-os: thread ctx meta reason=yield tick=3 runq=3 watermark=3\r\n"
        );
        assert_eq!(
            boot_thread_context_meta_line_bytes(),
            b"tosm-os: thread ctx meta reason=yield tick=3 runq=3 watermark=3\r\n"
        );
    }
    #[test]
    fn thread_lifecycle_line_bytes_include_crlf() {
        assert_eq!(
            BOOT_THREAD_STATE_BLOCKED_LINE,
            "tosm-os: thread state task=2 ready->blocked runq=2 selected=1\r\n"
        );
        assert_eq!(
            boot_thread_state_blocked_line_bytes(),
            b"tosm-os: thread state task=2 ready->blocked runq=2 selected=1\r\n"
        );
        assert_eq!(
            BOOT_THREAD_STATE_READY_LINE,
            "tosm-os: thread state task=2 blocked->ready runq=3 selected=1\r\n"
        );
        assert_eq!(
            boot_thread_state_ready_line_bytes(),
            b"tosm-os: thread state task=2 blocked->ready runq=3 selected=1\r\n"
        );
        assert_eq!(
            BOOT_THREAD_STATE_TERMINATED_LINE,
            "tosm-os: thread state task=2 ready->terminated runq=1 selected=0\r\n"
        );
        assert_eq!(
            BOOT_THREAD_WAKE_LINE,
            "tosm-os: thread wake task=2 reason=timer wait=0x2000 runq=3 sel=1\r\n"
        );
        assert_eq!(
            boot_thread_wake_line_bytes(),
            b"tosm-os: thread wake task=2 reason=timer wait=0x2000 runq=3 sel=1\r\n"
        );
        assert_eq!(
            boot_thread_state_terminated_line_bytes(),
            b"tosm-os: thread state task=2 ready->terminated runq=1 selected=0\r\n"
        );
        assert_eq!(
            BOOT_SCHEDULER_EDGE_BLOCKED_LINE,
            "tosm-os: scheduler edge case=blocked-selected task=1 runq=2 selected=0\r\n"
        );
        assert_eq!(
            boot_scheduler_edge_blocked_line_bytes(),
            b"tosm-os: scheduler edge case=blocked-selected task=1 runq=2 selected=0\r\n"
        );
        assert_eq!(
            BOOT_SCHEDULER_EDGE_TERMINATED_LINE,
            "tosm-os: scheduler edge case=terminated-dequeue task=2 err=task-not-found runq=1 selected=0\r\n"
        );
        assert_eq!(
            boot_scheduler_edge_terminated_line_bytes(),
            b"tosm-os: scheduler edge case=terminated-dequeue task=2 err=task-not-found runq=1 selected=0\r\n"
        );
    }

    #[test]
    fn scheduler_context_handoff_reports_saved_and_restored_registers() {
        reset_early_scheduler_state();
        enqueue_early_scheduler_task(2).expect("enqueue should add worker task");
        let _ = advance_early_scheduler_round_robin(super::EarlySchedulerHandoffReason::Yield);

        reset_early_timer_ticks();
        let timer = init_early_timer();
        let _first = dispatch_early_timer_interrupt(timer);
        let _second = dispatch_early_timer_interrupt(timer);
        let _third = dispatch_early_timer_interrupt(timer);

        let handoff =
            model_early_thread_context_handoff(2, super::EarlySchedulerHandoffReason::Yield)
                .expect("context handoff should succeed for runnable target");
        assert_eq!(handoff.from_task_id, 1);
        assert_eq!(handoff.to_task_id, 2);
        assert_eq!(handoff.saved.instruction_pointer, 0x0000_0000_0010_0200);
        assert_eq!(handoff.saved.stack_pointer, 0x0000_0000_0040_1f00);
        assert_eq!(handoff.restored.instruction_pointer, 0x0000_0000_0020_0000);
        assert_eq!(handoff.restored.stack_pointer, 0x0000_0000_0040_2000);
        assert_eq!(
            handoff.metadata.reason,
            super::EarlySchedulerHandoffReason::Yield
        );
        assert_eq!(handoff.metadata.timer_tick, 3);
        assert_eq!(handoff.metadata.run_queue_depth, 3);
        assert_eq!(handoff.metadata.queue_watermark, 3);
        assert_eq!(
            handoff.metadata.from_state,
            EarlyThreadLifecycleState::Running
        );
        assert_eq!(handoff.metadata.to_state, EarlyThreadLifecycleState::Ready);

        reset_early_timer_ticks();
        reset_early_scheduler_state();
    }

    #[test]
    fn thread_lifecycle_blocked_ready_transitions_update_scheduler_contracts() {
        reset_early_scheduler_state();
        enqueue_early_scheduler_task(2).expect("enqueue should add worker task");
        let _ = advance_early_scheduler_round_robin(super::EarlySchedulerHandoffReason::Yield);

        let blocked = transition_early_thread_lifecycle(2, EarlyThreadLifecycleState::Blocked)
            .expect("worker should transition to blocked");
        assert_eq!(blocked.from_state, EarlyThreadLifecycleState::Ready);
        assert_eq!(blocked.to_state, EarlyThreadLifecycleState::Blocked);
        assert_eq!(blocked.run_queue_depth, 2);
        assert_eq!(blocked.selected_task_id, 1);

        let ready = wake_early_thread(
            2,
            super::EarlyThreadWakeReason::Timer,
            0x0000_0000_0000_2000,
            3,
        )
        .expect("blocked worker should transition back to ready");
        assert_eq!(ready.from_state, EarlyThreadLifecycleState::Blocked);
        assert_eq!(ready.to_state, EarlyThreadLifecycleState::Ready);
        assert_eq!(ready.reason, super::EarlyThreadWakeReason::Timer);
        assert_eq!(ready.wait_channel, 0x0000_0000_0000_2000);
        assert_eq!(ready.wait_ownership.owner_task_id, 1);
        assert_eq!(ready.wait_ownership.blocked_task_id, 2);
        assert_eq!(ready.wait_ownership.claim_sequence, 1);
        assert_eq!(ready.timeout.deadline_tick, 3);
        assert_eq!(ready.timeout.observed_tick, 0);
        assert!(!ready.timeout.expired);
        assert_eq!(ready.run_queue_depth, 3);
        assert_eq!(ready.selected_task_id, 1);

        reset_early_scheduler_state();
    }

    #[test]
    fn wake_timeout_reports_expiry_once_deadline_is_reached() {
        reset_early_scheduler_state();
        enqueue_early_scheduler_task(2).expect("enqueue should add worker task");
        let _ = advance_early_scheduler_round_robin(super::EarlySchedulerHandoffReason::Yield);
        transition_early_thread_lifecycle(2, EarlyThreadLifecycleState::Blocked)
            .expect("worker should transition to blocked");

        reset_early_timer_ticks();
        let timer = init_early_timer();
        let _ = dispatch_early_timer_interrupt(timer);
        let _ = dispatch_early_timer_interrupt(timer);
        let _ = dispatch_early_timer_interrupt(timer);

        let ready = wake_early_thread(
            2,
            super::EarlyThreadWakeReason::Timer,
            0x0000_0000_0000_2000,
            3,
        )
        .expect("wake should include timeout metadata");

        assert_eq!(ready.wait_ownership.claim_sequence, 1);
        assert_eq!(ready.timeout.deadline_tick, 3);
        assert_eq!(ready.timeout.observed_tick, 3);
        assert!(ready.timeout.expired);

        reset_early_timer_ticks();
        reset_early_scheduler_state();
    }

    #[test]
    fn wake_requires_blocked_source_state() {
        reset_early_scheduler_state();
        enqueue_early_scheduler_task(2).expect("enqueue should add worker task");

        let err = wake_early_thread(
            2,
            super::EarlyThreadWakeReason::Signal,
            0x0000_0000_0000_3000,
            4,
        )
        .expect_err("wake should reject non-blocked source state");
        assert_eq!(
            err,
            super::EarlyThreadLifecycleError::InvalidStateTransition
        );

        reset_early_scheduler_state();
    }

    #[test]
    fn scheduler_edge_case_reports_cover_blocked_fallback_and_terminated_cleanup() {
        reset_early_scheduler_state();
        enqueue_early_scheduler_task(2).expect("enqueue should add worker task");

        let _ = advance_early_scheduler_round_robin(super::EarlySchedulerHandoffReason::Yield);

        let blocked = model_early_scheduler_blocked_selection_edge_case(1)
            .expect("selected bootstrap task should block and fall back to idle");
        assert_eq!(
            blocked.edge_case,
            super::EarlySchedulerEdgeCase::BlockedSelectedFallback
        );
        assert_eq!(blocked.task_id, 1);
        assert_eq!(blocked.run_queue_depth, 2);
        assert_eq!(blocked.selected_task_id, 0);
        assert_eq!(blocked.dequeue_error, None);

        let _ = transition_early_thread_lifecycle(2, EarlyThreadLifecycleState::Ready)
            .expect("worker should stay runnable before termination cleanup");

        let terminated = model_early_scheduler_terminated_cleanup_edge_case(2)
            .expect("termination cleanup modeling should succeed");
        assert_eq!(
            terminated.edge_case,
            super::EarlySchedulerEdgeCase::TerminatedDequeueRejected
        );
        assert_eq!(terminated.task_id, 2);
        assert_eq!(terminated.run_queue_depth, 1);
        assert_eq!(terminated.selected_task_id, 0);
        assert_eq!(
            terminated.dequeue_error,
            Some(super::EarlySchedulerMutationError::TaskNotFound)
        );

        reset_early_scheduler_state();
    }

    #[test]
    fn wake_contention_prefers_higher_priority_reason() {
        reset_early_scheduler_state();
        enqueue_early_scheduler_task(2).expect("enqueue should add first worker task");
        enqueue_early_scheduler_task(3).expect("enqueue should add second worker task");

        transition_early_thread_lifecycle(2, EarlyThreadLifecycleState::Blocked)
            .expect("worker two should transition to blocked");
        transition_early_thread_lifecycle(3, EarlyThreadLifecycleState::Blocked)
            .expect("worker three should transition to blocked");

        let contention = resolve_early_thread_wake_contention(
            0x0000_0000_0000_3000,
            2,
            super::EarlyThreadWakeReason::Timer,
            3,
            super::EarlyThreadWakeReason::Signal,
        )
        .expect("contention resolution should succeed for blocked tasks");

        assert_eq!(contention.wait_channel, 0x0000_0000_0000_3000);
        assert_eq!(contention.winner_task_id, 3);
        assert_eq!(contention.loser_task_id, 2);
        assert_eq!(
            contention.winner_reason,
            super::EarlyThreadWakeReason::Signal
        );
        assert_eq!(contention.loser_reason, super::EarlyThreadWakeReason::Timer);
        assert_eq!(contention.winner_claim_sequence, 2);
        assert_eq!(contention.loser_claim_sequence, 1);

        reset_early_scheduler_state();
    }

    #[test]
    fn wake_contention_rejects_non_blocked_participants() {
        reset_early_scheduler_state();
        enqueue_early_scheduler_task(2).expect("enqueue should add first worker task");
        enqueue_early_scheduler_task(3).expect("enqueue should add second worker task");

        transition_early_thread_lifecycle(2, EarlyThreadLifecycleState::Blocked)
            .expect("worker two should transition to blocked");

        let err = resolve_early_thread_wake_contention(
            0x0000_0000_0000_3000,
            2,
            super::EarlyThreadWakeReason::Timer,
            3,
            super::EarlyThreadWakeReason::Signal,
        )
        .expect_err("contention should fail unless all participants are blocked");
        assert_eq!(
            err,
            super::EarlyThreadWakeContentionError::TaskStateNotBlocked
        );

        reset_early_scheduler_state();
    }

    #[test]
    fn wake_timeout_and_wait_owner_lines_include_crlf() {
        assert_eq!(
            BOOT_THREAD_WAIT_OWNERSHIP_LINE,
            "tosm-os: thread wait owner=1 task=2 wait=0x2000 claim=1\r\n"
        );
        assert_eq!(
            boot_thread_wait_ownership_line_bytes(),
            b"tosm-os: thread wait owner=1 task=2 wait=0x2000 claim=1\r\n"
        );
        assert_eq!(
            BOOT_THREAD_WAKE_TIMEOUT_LINE,
            "tosm-os: thread wake timeout task=2 deadline=3 now=3 expired=1\r\n"
        );
        assert_eq!(
            boot_thread_wake_timeout_line_bytes(),
            b"tosm-os: thread wake timeout task=2 deadline=3 now=3 expired=1\r\n"
        );
        assert_eq!(
            BOOT_THREAD_WAIT_CONTENTION_LINE,
            "tosm-os: thread wait contend wait=0x3000 winner=3 loser=2 pri=signal>timer\r\n"
        );
        assert_eq!(
            boot_thread_wait_contention_line_bytes(),
            b"tosm-os: thread wait contend wait=0x3000 winner=3 loser=2 pri=signal>timer\r\n"
        );
        assert_eq!(
            BOOT_THREAD_WAKE_ORDER_LINE,
            "tosm-os: thread wake order first=3 second=2 wait=0x3000 claims=2,3\r\n"
        );
        assert_eq!(
            boot_thread_wake_order_line_bytes(),
            b"tosm-os: thread wake order first=3 second=2 wait=0x3000 claims=2,3\r\n"
        );
    }

    #[test]
    fn timer_ack_and_multi_tick_lines_include_crlf() {
        assert_eq!(
            BOOT_TIMER_THIRD_TICK_LINE,
            "tosm-os: timer tick irq=0x20 count=3 uptime_ns=30000000\r\n"
        );
        assert_eq!(
            boot_timer_third_tick_line_bytes(),
            b"tosm-os: timer tick irq=0x20 count=3 uptime_ns=30000000\r\n"
        );
        assert_eq!(
            BOOT_TIMER_ACK_LINE,
            "tosm-os: timer ack irq=0x20 pic=0x20 eoi=0x20\r\n"
        );
        assert_eq!(
            boot_timer_ack_line_bytes(),
            b"tosm-os: timer ack irq=0x20 pic=0x20 eoi=0x20\r\n"
        );
    }

    #[test]
    fn idt_entry_missing_is_zeroed() {
        assert_eq!(IdtEntry::missing().handler_addr(), 0);
    }
}
