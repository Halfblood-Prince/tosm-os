# Current milestone

- Active milestone: serial and screen output
- Subtask: add smoke automation coverage for VGA transcript scrolling behavior so CI enforces deterministic last-row lifecycle semantics alongside boot/panic ordering contracts
- Status: completed (smoke automation now executes targeted host tests that assert deterministic VGA boot/panic transcript ordering plus scrolling lifecycle contracts before QEMU runtime checks)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: extended `tools/smoke-test.sh` with an explicit transcript-test list that now includes the VGA scroll lifecycle model test, so smoke automation enforces deterministic last-row clear-and-scroll behavior in addition to boot/panic ordering checks.
- Next slice: add smoke automation coverage for VGA initialization and newline-row clearing behavior so CI enforces deterministic first-frame and destination-row hygiene contracts alongside ordering/scrolling guarantees.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `23009842256`
- Last tested commit: `6a961fdb2d712f98a0d5ef3d5a661ed68f8009c1`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
