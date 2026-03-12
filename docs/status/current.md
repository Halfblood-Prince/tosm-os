# Current milestone

- Active milestone: bootloader and entry
- Subtask: fix rustfmt failure for canonical efi_main completion-line test formatting
- Status: in_progress (formatted completion-line assertion to match rustfmt output; awaiting CI rerun)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `22997476568`
- Last tested commit: `7cacd1bf48682195e2140fe70492071f875945c7`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
