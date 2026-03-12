## Goal

Create the smallest bootable x86_64 Rust OS slice that can start in QEMU and emit a deterministic serial log line from the kernel entry path.

## Current state

- Repository now contains a Cargo workspace with a minimal `kernel` crate.
- The `kernel` crate exports deterministic boot serial line literals and byte-slice helpers used by firmware entry paths.
- The `boot/uefi-entry` crate now includes a UEFI application binary target (`bootx64`) in addition to its host-testable library.
- Active milestone is `bootloader and entry`.

## Constraints

- Keep each change buildable and reviewable.
- Stay on the active milestone only.
- Prefer the smallest correct slice over broader subsystem work.
- Update docs when behavior changes.
- Verification is delegated to GitHub Actions after merge.

## Design choice

Build the milestone incrementally:

- start with a tiny no_std `kernel` crate that defines the canonical boot banner
- add a separate UEFI entry crate that consumes this banner
- extend that crate to write the banner to COM1
- add smoke automation that first checks source contracts, then runs QEMU when firmware tooling is available
- centralize canonical CRLF-terminated lines in the kernel crate so firmware entry paths cannot drift

This keeps early milestone slices auditable and minimizes cross-cutting risk.

## Implementation steps

1. ✅ Create Cargo workspace and minimal host-testable `kernel` crate.
2. ✅ Implement UEFI entry crate that calls into kernel banner bytes.
3. ✅ Add a panic handler and COM1 serial writer in the UEFI entry crate.
4. ✅ Add initial smoke-test script for banner contract; keep it CI-portable with POSIX tooling and extend to QEMU runner in a later slice.
5. ✅ Wire scripts into `make` targets.
6. ✅ Update README and status docs for each slice.
7. ✅ Centralize the CRLF-terminated banner line in `kernel` and consume it from `boot/uefi-entry`.
8. ✅ Initialize COM1 UART line settings in `boot/uefi-entry` before transmitting banner bytes.
9. ✅ Centralize the early-boot panic serial line in `kernel` and consume it from `boot/uefi-entry` panic handler.
10. ✅ Add a canonical completion serial line in `kernel` and emit it from `boot/uefi-entry` before returning `EFI_SUCCESS`.
11. ✅ Extend the smoke contract check to require the canonical early-boot panic line alongside banner and completion lines.
12. ✅ Extend the smoke contract check to require explicit canonical CRLF line literals for banner, panic, and completion messages.
13. ✅ Add a UEFI `bootx64` application target and extend smoke automation to execute a QEMU boot check when QEMU + OVMF are available.
14. ✅ Gate `bootx64` binary crate attributes and entry symbol to `target_os = "uefi"`, with a host-only `main` shim so workspace host checks stop compiling UEFI no_std panic handlers as std test binaries.
15. ✅ Make QEMU smoke mandatory in CI by tightening smoke-test firmware discovery, adding additional common OVMF paths, and requiring QEMU execution under CI while preserving local best-effort smoke behavior.
16. ✅ Add smoke-script Rust target provisioning so `tools/smoke-test.sh` self-installs `x86_64-unknown-uefi` via `rustup` when missing, removing reliance on external pre-steps and fixing CI/local target-missing failures deterministically.
17. ✅ Fix UEFI smoke duplicate-entry linking by keeping `efi_main` exported only by the `bootx64` binary and routing shared logic through a library `run_entry` helper.
18. ✅ Fix smoke-script teardown robustness by avoiding `local run_dir` with an EXIT trap (which can leave `${run_dir}` unbound under `set -u`) and using a stable function-scope variable for temp-directory cleanup.
19. ✅ Add a minimal VGA text-mode writer in `boot/uefi-entry` and mirror canonical kernel banner/completion lines to on-screen output alongside COM1 serial logs.
20. ✅ Add deterministic VGA row lifecycle handling by clearing the screen at boot-log start and scrolling upward (instead of wrapping) when output reaches the last text row.
21. ✅ Add host-testable VGA writer model tests in `boot/uefi-entry` to validate init/newline/scroll semantics independent of memory-mapped hardware side effects.
22. ✅ Mirror the canonical early-boot panic line to VGA text output (after deterministic screen init) in the UEFI panic handler so panic diagnostics stay aligned across serial and screen channels.
23. ✅ Add host-testable VGA transcript ordering tests that validate boot banner/completion row ordering and panic-path screen reinitialization invariants.
24. ✅ Repair VGA transcript model assertions by trimming only trailing blank cells (instead of splitting at first blank) so canonical row lines with internal spaces compare correctly and CI fmt/tests pass.
25. ✅ Enforce canonical VGA transcript ordering contracts in smoke automation by executing targeted `uefi-entry` model transcript tests before QEMU runtime checks.

## Risks

- Missing `x86_64-unknown-uefi` target can block UEFI build/lint/smoke steps later.
- Missing QEMU/OVMF can block runtime smoke checks later.
- Hand-written ABI structs in the UEFI slice need careful layout validation.

## Verification steps

Expected GitHub Actions verification after merge:

- `make fmt`
- `make lint`
- `make test`
- `make build`
- `make smoke`
