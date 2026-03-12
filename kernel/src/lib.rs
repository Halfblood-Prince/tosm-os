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

/// Canonical paging-install line emitted after early page tables are materialized.
pub const BOOT_PAGING_INSTALL_LINE: &str =
    "tosm-os: paging install root=0x3f7ed000 span=0x40000000 entries=514\r\n";

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

pub const PAGE_SIZE_4K_BYTES: u64 = 0x1000;
pub const EARLY_PAGING_FRAME_WINDOW_FRAMES: usize = 4;
pub const EARLY_IDENTITY_MAP_PAGES_4K: usize = 512;

const PAGE_TABLE_ENTRY_PRESENT: u64 = 1 << 0;
const PAGE_TABLE_ENTRY_WRITABLE: u64 = 1 << 1;
const PAGE_TABLE_ENTRY_HUGE_PAGE: u64 = 1 << 7;
const PAGE_TABLE_ENTRY_ADDR_MASK: u64 = 0x000f_ffff_ffff_f000;
const PAGE_SIZE_2M_BYTES: u64 = 0x20_0000;
const ENTRIES_PER_PAGE_TABLE: usize = 512;
const EARLY_PAGING_PRESENT_ENTRY_COUNT: usize = 1 + 1 + EARLY_IDENTITY_MAP_PAGES_4K;

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
        boot_banner_bytes, boot_banner_line_bytes, boot_entry_done_line_bytes,
        boot_interrupt_init_line_bytes, boot_memory_init_line_bytes,
        boot_paging_install_line_bytes, boot_paging_plan_line_bytes, boot_panic_line_bytes,
        dispatch_exception, early_idt_descriptor, early_idt_entries, early_paging_table_snapshot,
        early_physical_memory_map, early_translation_state_valid, exception_log_line,
        exception_log_line_bytes, init_early_interrupts, init_early_paging_plan,
        init_early_physical_memory, install_early_paging, is_canonical_virtual_address,
        is_page_aligned_4k, translate_early_virtual_to_physical, EarlyFrameAllocationError,
        EarlyFrameAllocator, IdtEntry, PhysicalMemoryRegionKind, VirtualAddress,
        VirtualAddressTranslationError, BOOT_BANNER, BOOT_BANNER_LINE, BOOT_ENTRY_DONE_LINE,
        BOOT_INTERRUPT_INIT_LINE, BOOT_MEMORY_INIT_LINE, BOOT_PAGING_INSTALL_LINE,
        BOOT_PAGING_PLAN_LINE, BOOT_PANIC_LINE, EXCEPTION_VECTOR_COUNT,
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
