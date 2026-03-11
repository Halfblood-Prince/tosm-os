#!/usr/bin/env bash
set -euo pipefail

# Boot milestone placeholder smoke gate.
# This script intentionally checks for the deterministic boot banner contract
# until the QEMU boot path is added in a later slice.
expected_banner='tosm-os: kernel entry reached'

if ! grep --fixed-strings --quiet -- "${expected_banner}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
  echo "smoke: expected boot banner not found"
  exit 1
fi

echo "smoke: boot banner contract present"
