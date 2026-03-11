.PHONY: fmt lint test build qemu smoke ci

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all

build:
	cargo build --workspace

qemu:
	./tools/run-qemu.sh

smoke:
	./tools/smoke-test.sh

ci: fmt lint test build smoke
