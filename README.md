## Current boot slice

The repository is progressing through the first milestone (`bootloader and entry`).

This slice introduces a minimal Rust workspace with:

- a host-testable `kernel` crate that owns deterministic CRLF-terminated boot serial contracts, a first-pass x86_64 early IDT model, vector-specific exception stubs for vectors `0..31`, and canonical exception log line strings
- a `boot/uefi-entry` crate that defines a UEFI ABI `efi_main`, writes kernel-provided banner/interrupt-init/completion lines to COM1 and VGA text memory, includes a minimal panic handler, and provides a `bootx64` UEFI application target

Canonical boot banner:

- `tosm-os: kernel entry reached`

The smoke script (`tools/smoke-test.sh`) enforces deterministic serial message contracts and, when `qemu-system-x86_64` plus OVMF firmware images are available, boots the generated UEFI image in QEMU and verifies serial output.

Workspace validation commands are wired through canonical `make` targets:

- `make fmt`
- `make lint`
- `make test`
- `make build`
- `make smoke`

QEMU runtime smoke execution is mandatory in CI (`REQUIRE_QEMU_SMOKE=1`) and still opportunistic for local runs: local smoke checks fall back to contract-only validation when QEMU/OVMF are unavailable.

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
