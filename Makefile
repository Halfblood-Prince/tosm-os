.PHONY: fmt lint test build smoke

fmt:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-targets --all-features

build:
	cargo build --workspace

smoke:
	./tools/smoke-test.sh
