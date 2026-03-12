## Current boot slice

The repository is progressing through the first milestone (`bootloader and entry`).

This slice introduces a minimal Rust workspace with:

- a host-testable `kernel` crate that owns an explicit deterministic boot banner literal plus canonical CRLF-terminated serial line helpers for both entry and completion paths
- a `boot/uefi-entry` crate that defines a UEFI ABI `efi_main`, writes kernel-provided banner and completion lines to COM1, and includes a minimal panic handler

Canonical boot banner:

- `tosm-os: kernel entry reached`

A temporary smoke script (`tools/smoke-test.sh`) currently enforces the deterministic boot banner contract while QEMU automation is still pending.

Workspace validation commands are wired through canonical `make` targets:

- `make fmt`
- `make lint`
- `make test`
- `make build`
- `make smoke`

Full QEMU smoke automation remains the next incremental step.

## Codex + CI workflow

- Codex is expected to **write code and docs only**.
- Verification is delegated to GitHub Actions after a feature branch is merged into `main`.
- The `CI` workflow uploads logs and reports as artifacts.
- The `Project status` workflow copies the latest CI outcome, report summaries, and log excerpts into `docs/status/` on `main`.
- On the next Codex run, it should read `docs/status/current.md`, `docs/status/ci-feedback.json`, `docs/status/latest-ci.md`, and the generated reports/log excerpts before deciding whether to continue the milestone or fix a prior failure.

## Verification commands

These are the commands GitHub Actions is expected to run after merge:

- `make fmt`
- `make lint`
- `make test`
- `make build`
- `make smoke`
