# Current milestone

- Active milestone: bootloader and entry
- Subtask: add a minimal UEFI ABI entry stub crate wired to the kernel deterministic boot banner
- Status: ready_for_ci
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] cargo fmt --all
- [ ] cargo clippy --all-targets --all-features -- -D warnings
- [ ] cargo test --all
- [ ] boot smoke test in QEMU
- [x] docs updated

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: cancelled
- Last CI run: `22973095097`
- Last tested commit: `26e69e5f00ddbe196a9e00e0dc40506948edf08c`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
