#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::mem::size_of;
use core::sync::atomic::{AtomicBool, Ordering};

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

pub const PAGE_SIZE_4K_BYTES: u64 = 0x1000;
pub const EARLY_PAGING_FRAME_WINDOW_FRAMES: usize = 4;
pub const EARLY_IDENTITY_MAP_PAGES_4K: usize = 512;

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

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::{
        boot_banner_bytes, boot_banner_line_bytes, boot_entry_done_line_bytes,
        boot_interrupt_init_line_bytes, boot_memory_init_line_bytes, boot_paging_plan_line_bytes,
        boot_panic_line_bytes, dispatch_exception, early_idt_descriptor, early_idt_entries,
        early_physical_memory_map, exception_log_line, exception_log_line_bytes,
        init_early_interrupts, init_early_paging_plan, init_early_physical_memory, IdtEntry,
        PhysicalMemoryRegionKind, BOOT_BANNER, BOOT_BANNER_LINE, BOOT_ENTRY_DONE_LINE,
        BOOT_INTERRUPT_INIT_LINE, BOOT_MEMORY_INIT_LINE, BOOT_PAGING_PLAN_LINE, BOOT_PANIC_LINE,
        EXCEPTION_VECTOR_COUNT,
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
    fn idt_entry_missing_is_zeroed() {
        assert_eq!(IdtEntry::missing().handler_addr(), 0);
    }
}
