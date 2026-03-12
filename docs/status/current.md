# Current milestone

- Active milestone: serial and screen output
- Subtask: mirror canonical boot lines to VGA text output from UEFI entry
- Status: completed (`boot/uefi-entry` now writes the kernel-provided canonical entry and completion lines to both COM1 and VGA text memory, so early boot diagnostics are visible on serial and screen paths)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: implemented a minimal VGA text writer in `boot/uefi-entry` and wired `run_entry` to mirror canonical kernel banner and completion lines to screen output in addition to COM1 serial output.
- Next slice: keep milestone 2 moving by adding deterministic row management for the VGA path (for example clearing/initializing first lines before writing) while preserving canonical message parity with serial output.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `23002811149`
- Last tested commit: `c39e30fc5a36d31a272b17aa15b3aa61a4ef122a`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
