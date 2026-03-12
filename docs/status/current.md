# Current milestone

- Active milestone: bootloader and entry
- Subtask: resolve UEFI smoke link failure by ensuring only one exported `efi_main` symbol exists in `bootx64`
- Status: completed (`boot/uefi-entry` now exposes a shared `run_entry` function from the library, while only the `bootx64` binary exports the firmware `efi_main` symbol, eliminating the duplicate-symbol link error seen in smoke)
- Note: Codex writes code/docs only and waits for GitHub Actions feedback after merge to `main`.

## Done criteria
- [ ] make fmt
- [ ] make lint
- [ ] make test
- [ ] make build
- [ ] make smoke
- [x] docs updated

## Progress update

- Completed slice: fixed the smallest concrete CI failure in smoke (`rust-lld: error: duplicate symbol: efi_main`) by removing the exported UEFI entry symbol from the library crate, introducing a shared `run_entry` function, and keeping the exported `efi_main` symbol exclusively in the `bootx64` binary target.
- Next slice: wait for CI results for this linker fix; once CI is green, mark milestone 1 complete and begin milestone 2 (`serial and screen output`) with a minimal screen-output path mirroring canonical serial boot messages.

<!-- ci-status:start -->
## Latest CI automation

- Last CI conclusion: failure
- Last CI run: `23000801057`
- Last tested commit: `8253c809a4688d6e078e5d9368861ccbe7a1141a`
- Recommended next action for Codex: fix the smallest concrete failure from the latest CI excerpts before adding new scope
- Detailed summary: `docs/status/latest-ci.md`
<!-- ci-status:end -->
