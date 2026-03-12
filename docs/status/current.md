# Current milestone

- Active milestone: serial and screen output
- Subtask: mirror the canonical early-boot panic line to VGA text mode in addition to COM1 serial
- Status: completed (updated the UEFI panic path to initialize VGA boot logging and write the canonical panic line on screen as well as serial)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: extended `boot/uefi-entry` panic handling so panic diagnostics now mirror the canonical `tosm-os: panic in uefi-entry\r\n` line to VGA text output in addition to COM1, preserving deterministic early-boot diagnostics across serial and screen channels.
- Next slice: add host-testable coverage for complete boot-screen transcript ordering (banner, completion, and panic-path invariants) to protect deterministic serial/screen parity as milestone 2 progresses.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `23006078327`
- Last tested commit: `239359a7c32a87c29b89d6abb895107c9dcc42cb`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
