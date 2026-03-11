## Current boot slice

The repository currently targets the first milestone: a minimal x86_64 UEFI boot stub that writes a deterministic banner to COM1 under QEMU and then powers off.

## Prerequisites

- Rust stable with `rustfmt` and `clippy`
- Rust target `x86_64-unknown-uefi`
- `qemu-system-x86_64`
- OVMF firmware available in a standard QEMU installation path

## Verification commands

- `make fmt`
- `make lint`
- `make test`
- `make build`
- `make smoke`

## Notes

- `make build` builds the host-testable `kernel` crate and the UEFI boot artifact.
- `make smoke` stages `BOOTX64.EFI`, boots QEMU with OVMF, and asserts the serial banner `tosm-os: kernel entry reached` when prerequisites are present.
- `make qemu` and `make smoke` auto-select PowerShell scripts when available, and use POSIX shell scripts otherwise.

- `make lint`, `make build`, and `make smoke` print an explicit warning and skip UEFI-targeted steps when local prerequisites are missing.
