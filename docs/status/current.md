# Current milestone

- Active milestone: bootloader and entry
- Subtask: kernel entry stub boots in QEMU and writes to serial
- Status: in_progress
- Note: implementation is in progress; required local verification is currently blocked until `make`, `cargo`, and `qemu-system-x86_64` are available in the environment.

## Done criteria
- [ ] cargo fmt --all
- [ ] cargo clippy --all-targets --all-features -- -D warnings
- [ ] cargo test --all
- [ ] boot smoke test in QEMU
- [ ] docs updated
