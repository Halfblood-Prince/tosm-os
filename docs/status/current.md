# Current milestone

- Active milestone: bootloader and entry
- Subtask: make QEMU smoke mandatory in CI with deterministic OVMF handling while keeping local smoke checks portable
- Status: completed (CI smoke step now requires QEMU execution and smoke tooling handles broader OVMF layouts plus per-run mutable vars copies)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: made CI smoke execution mandatory by running `tools/smoke-test.sh` with `REQUIRE_QEMU_SMOKE=1`, expanded OVMF discovery for common Linux layouts (including 4M images), and switched QEMU to use a temporary copy of OVMF vars for deterministic, isolated runs.
- Next slice: start milestone 2 (`serial and screen output`) by adding a minimal screen-output path (framebuffer or UEFI console) that mirrors the canonical early boot messages currently emitted on serial.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `22999437124`
- Last tested commit: `7c535cbe021952e9b0c7dcb0b1dd4ee59a4bfa2a`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
