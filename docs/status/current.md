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

- Last CI conclusion: success
- Last CI run: `23002811149`
- Last tested commit: `c39e30fc5a36d31a272b17aa15b3aa61a4ef122a`
- Recommended next action for Codex: continue the next unfinished milestone slice; do not redo already-green validation work
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
