.PHONY: fmt lint test build qemu smoke ci

POWERSHELL := $(shell command -v powershell || command -v pwsh)
CARGO := cargo +1.92.0-x86_64-unknown-linux-gnu

fmt:
	$(CARGO) fmt --all

lint:
	$(CARGO) clippy -p kernel --all-targets --all-features -- -D warnings
	@if ./tools/preflight.sh rust-target; then \
		$(CARGO) clippy --manifest-path boot/uefi/Cargo.toml --target x86_64-unknown-uefi -- -D warnings; \
	else \
		echo "warning: skipping boot/uefi clippy; missing rust target x86_64-unknown-uefi"; \
	fi

test:
	$(CARGO) test -p kernel

build:
	$(CARGO) build -p kernel
	@if ./tools/preflight.sh rust-target; then \
		$(CARGO) build --manifest-path boot/uefi/Cargo.toml --target x86_64-unknown-uefi --target-dir build/cargo-target; \
	else \
		echo "warning: skipping boot/uefi build; missing rust target x86_64-unknown-uefi"; \
	fi

qemu:
ifdef POWERSHELL
	$(POWERSHELL) -ExecutionPolicy Bypass -File tools/run-qemu.ps1
else
	./tools/run-qemu.sh
endif

smoke:
	@if ! ./tools/preflight.sh rust-target; then \
		echo "warning: skipping smoke; missing rust target x86_64-unknown-uefi"; \
	elif ! ./tools/preflight.sh qemu; then \
		echo "warning: skipping smoke; qemu-system-x86_64 not found"; \
	elif ! ./tools/preflight.sh ovmf; then \
		echo "warning: skipping smoke; OVMF firmware not found"; \
	elif [ -n "$(POWERSHELL)" ]; then \
		$(POWERSHELL) -ExecutionPolicy Bypass -File tools/smoke-test.ps1; \
	else \
		./tools/smoke-test.sh; \
	fi

ci: fmt lint test build smoke
