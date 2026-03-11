# Current milestone

- Active milestone: bootloader and entry
- Subtask: kernel entry stub boots in QEMU and writes to serial
- Status: awaiting_ci_feedback
- Note: Codex should write code/docs only, then rely on GitHub Actions after merge to `main`. Read the latest CI automation section below before starting the next slice.

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
