# Current milestone

- Active milestone: bootloader and entry
- Subtask: make smoke self-provision the UEFI Rust target so EFI image builds do not fail when `x86_64-unknown-uefi` is missing
- Status: completed (`tools/smoke-test.sh` now checks installed Rust targets and runs `rustup target add x86_64-unknown-uefi` before building `bootx64`, so smoke no longer depends on external target pre-provisioning)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: fixed the smallest concrete CI failure in smoke by adding target self-provisioning inside `tools/smoke-test.sh`; the script now installs `x86_64-unknown-uefi` via `rustup` when absent before building `bootx64`, directly addressing the reported `can't find crate for core` EFI compilation error.
- Next slice: wait for CI results for this smoke fix; once CI is green, mark milestone 1 complete and begin milestone 2 (`serial and screen output`) with a minimal screen-output path mirroring canonical serial boot messages.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `22999437124`
- Last tested commit: `7c535cbe021952e9b0c7dcb0b1dd4ee59a4bfa2a`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
