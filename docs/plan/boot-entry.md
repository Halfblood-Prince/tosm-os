## Goal

Create the smallest bootable x86_64 Rust OS slice that can start in QEMU and emit a deterministic serial log line from the kernel entry path.

## Current state

- Repository now contains a Cargo workspace with a minimal `kernel` crate.
- The `kernel` crate exports a deterministic boot banner literal and byte-slice helper for future firmware serial output wiring.
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
- only then add QEMU smoke automation tied to the expected serial output

This keeps early milestone slices auditable and minimizes cross-cutting risk.

## Implementation steps

1. ✅ Create Cargo workspace and minimal host-testable `kernel` crate.
2. 🟡 Implement UEFI entry stub crate that calls into kernel banner bytes.
3. ⏳ Add panic handler and COM1 serial writer in the UEFI entry crate.
4. ✅ Add initial smoke-test script for banner contract; keep it CI-portable with POSIX tooling and extend to QEMU runner in a later slice.
5. ⏳ Wire scripts into `make` targets.
6. ⏳ Update README and status docs for each slice.

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
