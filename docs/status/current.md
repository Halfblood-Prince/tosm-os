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

- Last CI conclusion: success
- Last CI run: `22974012535`
- Last tested commit: `58046383da594b9e56e83b4edd68d8a5cb436850`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
