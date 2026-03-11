# Current milestone

- Active milestone: bootloader and entry
- Subtask: remove the unused COM1 line-status constant that fails clippy with `-D warnings`
- Status: ready_for_ci (awaiting CI rerun on this fix)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] cargo fmt --all
- [ ] cargo clippy --all-targets --all-features -- -D warnings
- [ ] cargo test --all
- [ ] boot smoke test in QEMU
- [x] docs updated

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `22974646500`
- Last tested commit: `4109da5a796140d5ed7f861dd039ed2f34717e64`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
