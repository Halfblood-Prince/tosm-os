#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

/// Deterministic serial banner used by the boot milestone smoke test.
pub const BOOT_BANNER: &str = "tosm-os: kernel entry reached";

/// Canonical serial line emitted from boot entry paths.
pub const BOOT_BANNER_LINE: &str = "tosm-os: kernel entry reached\r\n";

/// Canonical panic line emitted by early boot firmware entry paths.
pub const BOOT_PANIC_LINE: &str = "tosm-os: panic in uefi-entry\r\n";

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

/// Returns the canonical panic line (including CRLF) for early serial panic paths.
#[must_use]
pub const fn boot_panic_line_bytes() -> &'static [u8] {
    BOOT_PANIC_LINE.as_bytes()
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::{
        boot_banner_bytes, boot_banner_line_bytes, boot_panic_line_bytes, BOOT_BANNER,
        BOOT_BANNER_LINE, BOOT_PANIC_LINE,
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
    fn boot_panic_line_bytes_include_crlf() {
        assert_eq!(BOOT_PANIC_LINE, "tosm-os: panic in uefi-entry\r\n");
        assert_eq!(boot_panic_line_bytes(), b"tosm-os: panic in uefi-entry\r\n");
    }
}
