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

- Last CI conclusion: unknown
- Last CI run: `unknown`
- Last tested commit: `unknown`
- Recommended next action for Codex: read the generated CI files after the first post-merge automation run
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
