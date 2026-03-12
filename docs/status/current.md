# Current milestone

- Active milestone: serial and screen output
- Subtask: add smoke automation coverage for VGA carriage-return and row-wrap behavior so CI enforces deterministic in-row overwrite and width-boundary newline semantics alongside init/newline/ordering/scrolling contracts
- Status: completed (smoke automation now executes targeted host tests that assert deterministic VGA initialization, newline destination-row clearing, transcript ordering, and last-row scrolling lifecycle contracts before QEMU runtime checks)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: extended `tools/smoke-test.sh` transcript test list to include the VGA initialization reset and newline destination-row clearing model tests, so smoke automation now gates first-frame hygiene and newline row-clearing behavior in addition to transcript ordering and scrolling semantics.
- Next slice: add smoke automation coverage for VGA carriage-return overwrite and row-wrap behavior so CI enforces deterministic in-row reset and width-boundary line-advance semantics.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: cancelled
- Last CI run: `23009735086`
- Last tested commit: `a778ec5cadda305cb83905d249b81a50a2c84fd2`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
