# Current milestone

- Active milestone: bootloader and entry
- Subtask: kernel entry stub boots in QEMU and writes to serial
- Status: in_progress
- Note: `make` now runs host checks by default and skips UEFI-targeted verification with explicit warnings when `x86_64-unknown-uefi`, QEMU, or OVMF prerequisites are unavailable.

## Done criteria
- [x] cargo fmt --all
- [ ] cargo clippy --all-targets --all-features -- -D warnings
- [x] cargo test --all
- [ ] boot smoke test in QEMU
- [x] docs updated
