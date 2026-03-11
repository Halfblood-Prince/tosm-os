# Boot entry stub

## Purpose

Provide the first bootable x86_64 slice for `tosm-os`: a UEFI entry stub that emits a deterministic serial banner and then powers the VM off.

## Key data structures

- `kernel::boot_banner()`: stable banner string shared between tests and the boot path.
- `SerialPort`: minimal COM1 UART writer used by the UEFI stub and panic path.

## Invariants

- Serial output is limited to COM1 (`0x3F8`) in QEMU-compatible 16550 mode.
- The boot artifact is staged as `EFI/BOOT/BOOTX64.EFI`.
- The smoke test asserts the exact banner `tosm-os: kernel entry reached`.

## Failure modes

- Missing `x86_64-unknown-uefi` Rust target prevents the boot artifact from building.
- Missing QEMU or OVMF firmware prevents the smoke test from running.
- If serial output changes, the smoke assertion fails closed.

## Testing approach

- Unit test the shared boot banner in the `kernel` crate.
- Build the UEFI boot stub for `x86_64-unknown-uefi`.
- Boot QEMU under OVMF and assert the serial log line.
