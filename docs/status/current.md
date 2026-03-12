# Current milestone

- Active milestone: bootloader and entry
- Subtask: make CI smoke build UEFI target successfully by provisioning `x86_64-unknown-uefi` in the smoke job toolchain setup
- Status: completed (boot-smoke CI now installs the UEFI compilation target before invoking `tools/smoke-test.sh`)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: fixed the smallest concrete CI failure in smoke by adding `x86_64-unknown-uefi` target installation to `.github/workflows/ci.yml` `boot-smoke` Rust toolchain setup, addressing the reported `can't find crate for core` error during EFI target compilation.
- Next slice: rerun CI and, on success, begin milestone 2 (`serial and screen output`) by adding a minimal screen-output path that mirrors canonical serial boot messages.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `22999751292`
- Last tested commit: `db4792faf077f6e96411a3786652f050f3b2f8ed`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
