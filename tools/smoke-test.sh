#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/common.sh"

ROOT=$(repo_root)
TARGET_DIR="$ROOT/build/cargo-target"
EXPECTED='tosm-os: kernel entry reached'

${CARGO_BIN:-cargo +1.92.0-x86_64-unknown-linux-gnu} build \
  --manifest-path "$ROOT/boot/uefi/Cargo.toml" \
  --target x86_64-unknown-uefi \
  --target-dir "$TARGET_DIR"

"$ROOT/tools/run-qemu.sh"

SERIAL_LOG="$ROOT/build/qemu-serial.log"
if [[ ! -f "$SERIAL_LOG" ]]; then
  echo "Smoke test failed: serial log was not produced" >&2
  exit 1
fi

if ! grep -Fq "$EXPECTED" "$SERIAL_LOG"; then
  echo "Smoke test failed: expected serial banner '$EXPECTED' was not found" >&2
  exit 1
fi

echo "smoke ok"
