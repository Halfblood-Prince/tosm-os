## Goal

Create the smallest bootable x86_64 Rust OS slice that can start in QEMU and emit a deterministic serial log line from the kernel entry path.

## Current state

- Repository contains a Cargo workspace with `kernel` and `boot/uefi` crates.
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

- a host-testable `kernel` crate that owns the deterministic banner literal
- a `boot/uefi` no_std firmware entry that performs serial output and shutdown via raw UEFI ABI structures
- no external crates in the boot path to keep offline builds and auditing simpler
- dual smoke/QEMU scripts (PowerShell and POSIX shell) selected by `make`
- preflight gating so required `make` commands stay deterministic in constrained environments while reporting skipped UEFI checks

This keeps the first milestone narrow while leaving a clean path for later interrupt and memory work.

## Implementation steps

1. Keep the workspace and kernel crate minimal and deterministic.
2. Implement the UEFI entry stub, panic handler, and COM1 serial writer.
3. Add QEMU runner and smoke-test scripts for both PowerShell and POSIX shells.
4. Wire scripts into the existing `Makefile` with automatic host-shell selection.
5. Add prerequisite checks that gate UEFI-specific verification with explicit skip warnings.
6. Update README and status docs for the current workflow and blockers.
7. Run required verification and fix the smallest root cause if anything fails.

## Risks

- Missing `x86_64-unknown-uefi` target blocks UEFI build/lint/smoke steps.
- Missing QEMU or OVMF firmware blocks runtime smoke tests.
- Manual UEFI ABI definitions are sensitive to layout mistakes and require strict `repr(C)` correctness.

## Verification steps

- `make fmt`
- `make lint`
- `make test`
- `make build`
- `make smoke`
