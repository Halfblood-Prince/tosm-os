#![no_std]
#![forbid(unsafe_op_in_unsafe_fn)]

use core::ffi::c_void;

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

/// Returns the deterministic kernel message expected by boot milestone consumers.
#[must_use]
pub const fn kernel_entry_message() -> &'static [u8] {
    kernel::boot_banner_bytes()
}

/// UEFI ABI entrypoint stub for the boot milestone.
///
/// The next slice will use the provided table pointer to write this message to serial output.
#[no_mangle]
pub extern "efiapi" fn efi_main(_image: EfiHandle, _system_table: EfiSystemTable) -> EfiStatus {
    let _message = kernel_entry_message();
    EfiStatus::SUCCESS
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::{efi_main, kernel_entry_message, EfiHandle, EfiStatus, EfiSystemTable};

    #[test]
    fn entry_message_matches_kernel_banner() {
        assert_eq!(kernel_entry_message(), b"tosm-os: kernel entry reached");
    }

    #[test]
    fn efi_main_returns_success_in_stub_slice() {
        let status = efi_main(EfiHandle(core::ptr::null_mut()), EfiSystemTable(core::ptr::null_mut()));
        assert_eq!(status, EfiStatus::SUCCESS);
    }
}
