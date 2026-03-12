#![cfg_attr(target_os = "uefi", no_std)]
#![cfg_attr(target_os = "uefi", no_main)]
#![forbid(unsafe_op_in_unsafe_fn)]

#[cfg(target_os = "uefi")]
use uefi_entry::{run_entry, EfiHandle, EfiStatus, EfiSystemTable};

/// UEFI firmware entry symbol for the boot application image.
#[cfg(target_os = "uefi")]
#[no_mangle]
pub extern "efiapi" fn efi_main(handle: EfiHandle, system_table: EfiSystemTable) -> EfiStatus {
    run_entry(handle, system_table)
}

/// Host-only shim to keep workspace host checks buildable without UEFI ABI/linker semantics.
#[cfg(not(target_os = "uefi"))]
fn main() {}
