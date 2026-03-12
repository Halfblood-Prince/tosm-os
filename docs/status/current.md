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

- Last CI conclusion: success
- Last CI run: `22999092615`
- Last tested commit: `22cbfd4452fc9534783472399e5865512f4abb56`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
