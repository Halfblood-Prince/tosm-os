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

/// Number of architectural exception vectors reserved at boot.
pub const EXCEPTION_VECTOR_COUNT: usize = 32;

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
    pub handler_addr: usize,
}

extern "C" fn early_exception_stub() {
    loop {
        core::hint::spin_loop();
    }
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
        let handler = early_exception_stub as usize;
        let mut index = 0;
        while index < EXCEPTION_VECTOR_COUNT {
            EARLY_IDT[index] = IdtEntry::interrupt_gate(handler);
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
        handler_addr: early_exception_stub as usize,
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
        boot_interrupt_init_line_bytes, boot_panic_line_bytes, early_idt_descriptor,
        early_idt_entries, init_early_interrupts, IdtEntry, BOOT_BANNER, BOOT_BANNER_LINE,
        BOOT_ENTRY_DONE_LINE, BOOT_INTERRUPT_INIT_LINE, BOOT_PANIC_LINE, EXCEPTION_VECTOR_COUNT,
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
    fn idt_entries_point_to_common_early_stub() {
        let first = early_idt_entries()[0].handler_addr();
        for entry in early_idt_entries() {
            assert_eq!(entry.handler_addr(), first);
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
        assert_eq!(report.handler_addr, early_idt_entries()[0].handler_addr());
        assert_eq!(report.descriptor, early_idt_descriptor());
    }

    #[test]
    fn idt_entry_missing_is_zeroed() {
        assert_eq!(IdtEntry::missing().handler_addr(), 0);
    }
}
