.PHONY: fmt lint test build qemu smoke ci

POWERSHELL := $(shell command -v powershell || command -v pwsh)
CARGO := cargo +1.92.0-x86_64-unknown-linux-gnu

fmt:
	$(CARGO) fmt --all

lint:
	$(CARGO) clippy -p kernel --all-targets --all-features -- -D warnings
	$(CARGO) clippy --manifest-path boot/uefi/Cargo.toml --target x86_64-unknown-uefi -- -D warnings

test:
	$(CARGO) test -p kernel

build:
	$(CARGO) build -p kernel
	$(CARGO) build --manifest-path boot/uefi/Cargo.toml --target x86_64-unknown-uefi --target-dir build/cargo-target

qemu:
ifdef POWERSHELL
	$(POWERSHELL) -ExecutionPolicy Bypass -File tools/run-qemu.ps1
else
	./tools/run-qemu.sh
endif

smoke:
ifdef POWERSHELL
	$(POWERSHELL) -ExecutionPolicy Bypass -File tools/smoke-test.ps1
else
	./tools/smoke-test.sh
endif

ci: fmt lint test build smoke
