# Current milestone

- Active milestone: bootloader and entry
- Subtask: centralize canonical boot banner line (with CRLF) in kernel for UEFI serial output reuse
- Status: ready_for_ci (awaiting CI run on canonical banner-line centralization)
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
- Last CI run: `22975354212`
- Last tested commit: `727ffe36c3e415559fa44cffa496957121e65c3e`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
