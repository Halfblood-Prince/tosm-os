## Goal

Create the smallest bootable x86_64 Rust OS slice that can start in QEMU and emit a deterministic serial log line from the kernel entry path.

## Current state

- Repository contains only process docs and a `Makefile`.
- No Cargo workspace, kernel crate, boot configuration, or smoke-test tooling exists.
- Active milestone is `bootloader and entry`.
- Active subtask is `kernel entry stub boots in QEMU and writes to serial`.

## Constraints

- Keep the change buildable and testable.
- Stay on the active milestone only.
- Prefer the smallest correct slice over broader subsystem work.
- Update docs when behavior changes.
- Verification must run through `make fmt`, `make lint`, `make test`, `make build`, and `make smoke`.

## Design choice

Use a minimal Cargo workspace with:

- a `kernel` freestanding binary crate
- the `bootloader` toolchain path to produce a bootable disk image
- serial port output as the only runtime behavior
- a host-side smoke script that boots QEMU, captures serial output, and asserts the expected banner

This keeps the first milestone narrow while leaving a clean path for later interrupt and memory work.

## Implementation steps

1. Create the Cargo workspace and kernel crate metadata.
2. Add the kernel entry stub, panic handler, and serial writer.
3. Add build config and runner tooling to produce a QEMU-bootable image.
4. Add smoke-test and QEMU helper scripts wired into the existing `Makefile`.
5. Update the README and milestone status docs for the new workflow.
6. Run required verification and fix the smallest root cause if anything fails.

## Risks

- External Rust crates may need to be downloaded before verification can pass.
- QEMU or OVMF firmware may be unavailable on the host environment.
- Bootloader integration can require target-specific configuration that differs across environments.

## Verification steps

- `make fmt`
- `make lint`
- `make test`
- `make build`
- `make smoke`
