# Current milestone

- Active milestone: bootloader and entry
- Subtask: add a minimal COM1 serial writer in the UEFI entry path so firmware can emit the kernel banner
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

- Last CI conclusion: failure
- Last CI run: `22974365862`
- Last tested commit: `90dcbbfb18e508cbbc362adae6cf10c6d625e6b1`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
