# Current milestone

- Active milestone: bootloader and entry
- Subtask: kernel entry stub boots in QEMU and writes to serial
- Status: in_progress
- Note: local `make` and host `cargo` checks run, but UEFI-targeted checks are blocked in this environment until `rustup target add x86_64-unknown-uefi` can download the standard library.

## Done criteria
- [x] cargo fmt --all
- [ ] cargo clippy --all-targets --all-features -- -D warnings
- [x] cargo test --all
- [ ] boot smoke test in QEMU
- [x] docs updated
