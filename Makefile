.PHONY: fmt lint test build qemu smoke ci

fmt:
	cargo fmt --all

lint:
	cargo clippy -p kernel --all-targets --all-features -- -D warnings
	cargo clippy --manifest-path boot/uefi/Cargo.toml --target x86_64-unknown-uefi -- -D warnings

test:
	cargo test -p kernel

build:
	cargo build -p kernel
	cargo build --manifest-path boot/uefi/Cargo.toml --target x86_64-unknown-uefi --target-dir build/cargo-target

qemu:
	powershell -ExecutionPolicy Bypass -File tools/run-qemu.ps1

smoke:
	powershell -ExecutionPolicy Bypass -File tools/smoke-test.ps1

ci: fmt lint test build smoke
