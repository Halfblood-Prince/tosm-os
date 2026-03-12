#!/usr/bin/env bash
set -euo pipefail

# Boot milestone placeholder smoke gate.
# This script intentionally checks for the deterministic boot banner contract
# until the QEMU boot path is added in a later slice.
expected_banner='tosm-os: kernel entry reached'
expected_panic='tosm-os: panic in uefi-entry'
expected_entry_done='tosm-os: efi_main completed'
expected_banner_line='tosm-os: kernel entry reached\r\n'
expected_panic_line='tosm-os: panic in uefi-entry\r\n'
expected_entry_done_line='tosm-os: efi_main completed\r\n'

if ! grep --fixed-strings --quiet -- "${expected_banner}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
  echo "smoke: expected boot banner not found"
  exit 1
fi

if ! grep --fixed-strings --quiet -- "${expected_entry_done}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
  echo "smoke: expected efi_main completion line not found"
  exit 1
fi

if ! grep --fixed-strings --quiet -- "${expected_panic}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
  echo "smoke: expected panic line not found"
  exit 1
fi

if ! grep --fixed-strings --quiet -- "${expected_banner_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
  echo "smoke: expected boot banner CRLF contract not found"
  exit 1
fi

if ! grep --fixed-strings --quiet -- "${expected_panic_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
  echo "smoke: expected panic CRLF contract not found"
  exit 1
fi

if ! grep --fixed-strings --quiet -- "${expected_entry_done_line}" kernel/src/lib.rs boot/uefi-entry/src/lib.rs; then
  echo "smoke: expected efi_main completion CRLF contract not found"
  exit 1
fi

echo "smoke: boot banner, panic, completion, and CRLF contracts present"
