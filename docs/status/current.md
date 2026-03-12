# Current milestone

- Active milestone: bootloader and entry
- Subtask: fix smoke script unbound-variable failure after QEMU execution in CI
- Status: completed (`tools/smoke-test.sh` now uses a stable temp-directory variable for the EXIT cleanup trap so cleanup no longer references an out-of-scope `local run_dir` under `set -u`, removing the `run_dir: unbound variable` smoke failure)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: fixed the smallest concrete smoke failure (`./tools/smoke-test.sh: line 1: run_dir: unbound variable`) by replacing the function-local `run_dir` temp directory with a stable variable used by the EXIT trap and all QEMU runtime paths.
- Next slice: wait for CI results for this smoke-script fix; once CI is green, mark milestone 1 complete and begin milestone 2 (`serial and screen output`) with a minimal screen-output path mirroring canonical serial boot messages.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `23000801057`
- Last tested commit: `8253c809a4688d6e078e5d9368861ccbe7a1141a`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
