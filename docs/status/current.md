# Current milestone

- Active milestone: serial and screen output
- Subtask: make VGA boot output deterministic by initializing screen state and replacing row wraparound with explicit scroll + row clear behavior
- Status: completed (added explicit VGA init/clear behavior plus deterministic newline handling that clears destination rows and scrolls upward at the bottom)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: implemented deterministic VGA row management in `boot/uefi-entry` by clearing the text buffer at entry, clearing each destination row on newline, and scrolling screen contents upward at the bottom instead of wrapping to row 0.
- Next slice: continue milestone 2 by adding a small host-testable VGA writer model in `boot/uefi-entry` tests so newline/scroll semantics are validated independent of hardware memory side effects.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `23004069384`
- Last tested commit: `20f471312a79ff05f820a35987e0b96023cf1d2a`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
