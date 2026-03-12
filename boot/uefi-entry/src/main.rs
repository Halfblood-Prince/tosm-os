#![no_std]
#![no_main]
#![forbid(unsafe_op_in_unsafe_fn)]

use uefi_entry::{efi_main as uefi_crate_entry, EfiHandle, EfiStatus, EfiSystemTable};

/// UEFI firmware entry symbol for the boot application image.
#[no_mangle]
pub extern "efiapi" fn efi_main(handle: EfiHandle, system_table: EfiSystemTable) -> EfiStatus {
    uefi_crate_entry(handle, system_table)
}
