# Current milestone

- Active milestone: bootloader and entry
- Subtask: add a UEFI application target and opportunistic QEMU smoke execution over serial output
- Status: in_progress (implemented `bootx64` UEFI target and smoke script now runs QEMU when firmware tooling exists)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: UEFI binary target (`bootx64`) is now defined for `x86_64-unknown-uefi`, and smoke now does both source-level contract checks and runtime serial validation in QEMU when `qemu-system-x86_64` + OVMF are available.
- Next slice: make QEMU smoke mandatory in CI by provisioning deterministic OVMF firmware paths and enforcing runtime execution instead of fallback mode.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `22998781283`
- Last tested commit: `6bd5cb19deb0f760e946b0f55964669a0850d8da`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
