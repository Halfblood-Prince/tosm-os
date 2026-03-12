# Current milestone

- Active milestone: serial and screen output
- Subtask: enforce canonical boot-screen transcript ordering contracts in smoke automation so CI checks row-ordering semantics in addition to literal message presence
- Status: completed (smoke automation now executes targeted host tests that assert deterministic VGA boot and panic transcript ordering contracts before QEMU runtime checks)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: extended `tools/smoke-test.sh` with a dedicated screen transcript contract phase that runs targeted `uefi-entry` model tests for boot-line ordering and panic-path reinitialization invariants, then reports an explicit VGA contract success marker.
- Next slice: add smoke automation coverage for VGA transcript scrolling behavior so CI enforces deterministic last-row lifecycle semantics alongside boot/panic row-order guarantees.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: success
- Last CI run: `23008206147`
- Last tested commit: `8731e9398778dc5310b7388dfdb4fe98e75cde7a`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
