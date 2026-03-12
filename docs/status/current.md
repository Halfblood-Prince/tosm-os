# Current milestone

- Active milestone: serial and screen output
- Subtask: add a host-testable VGA writer model that validates deterministic init/newline/scroll behavior without touching memory-mapped VGA hardware
- Status: completed (added an in-test VGA writer model and assertions for init screen clear, newline row clearing, and bottom-row scroll semantics)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: added a host-testable `VgaWriterModel` in `boot/uefi-entry` tests and covered three deterministic behaviors: boot-log init clears screen + resets cursor, newline clears destination row, and bottom-row newline scrolls upward then clears the last row.
- Next slice: continue milestone 2 by extending on-screen diagnostics to cover the early panic path (mirror canonical panic line to VGA in addition to serial) while preserving deterministic row behavior.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `23004556312`
- Last tested commit: `2c7e943b58172267bff4ccacb8039f988e4c9070`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
