## Current boot slice

The repository currently targets the first milestone: a minimal x86_64 UEFI boot stub that writes a deterministic banner to COM1 under QEMU and then powers off.

## Codex + CI workflow

- Codex is expected to **write code and docs only**.
- Verification is delegated to GitHub Actions after a feature branch is merged into `main`.
- The `CI` workflow uploads logs and reports as artifacts.
- The `Project status` workflow copies the latest CI outcome, report summaries, and log excerpts into `docs/status/` on `main`.
- On the next Codex run, it should read `docs/status/current.md`, `docs/status/ci-feedback.json`, `docs/status/latest-ci.md`, and the generated reports/log excerpts before deciding whether to continue the milestone or fix a prior failure.

## Prerequisites

- Rust stable with `rustfmt` and `clippy`
- Rust target `x86_64-unknown-uefi`
- `qemu-system-x86_64`
- OVMF firmware available in a standard QEMU installation path

## Verification commands

These are the commands GitHub Actions is expected to run after merge:

- `make fmt`
- `make lint`
- `make test`
- `make build`
- `make smoke`

## Notes

- `make build` builds the host-testable `kernel` crate and the UEFI boot artifact.
- `make smoke` stages `BOOTX64.EFI`, boots QEMU with OVMF, and asserts the serial banner `tosm-os: kernel entry reached`.
- `make qemu` and `make smoke` auto-select PowerShell scripts when available, and use POSIX shell scripts otherwise.
