# Current milestone

- Active milestone: bootloader and entry
- Subtask: fix smoke CI regression by making the banner smoke check portable (remove rg dependency)
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
- Last CI run: `22973495491`
- Last tested commit: `ff9f25b873855c5a3a76bb1f78b061f02b253352`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
