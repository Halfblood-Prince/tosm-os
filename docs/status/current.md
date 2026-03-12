# Current milestone

- Active milestone: bootloader and entry
- Subtask: make the `bootx64` UEFI binary target host-check friendly so workspace CI jobs can compile/test/clippy without pulling UEFI panic semantics into host `std` test builds
- Status: in_progress (implemented target-gated UEFI entry attributes and symbol export, with a host-only shim `main` for non-UEFI checks)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: fixed the smallest concrete CI blocker by gating `boot/uefi-entry/src/main.rs` to UEFI-only `no_std`/`no_main` + `efi_main`, while providing a host-only `main()` shim so host-target `clippy`, `test`, and `build` no longer compile the UEFI panic runtime as a `std` test binary.
- Next slice: rerun CI and, if green on host checks, continue the milestone plan to make QEMU smoke mandatory with deterministic OVMF provisioning in CI.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `22998781283`
- Last tested commit: `6bd5cb19deb0f760e946b0f55964669a0850d8da`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
