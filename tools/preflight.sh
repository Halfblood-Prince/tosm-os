#!/usr/bin/env bash
set -euo pipefail

check_rust_target() {
  rustup target list --installed 2>/dev/null | grep -Fxq "x86_64-unknown-uefi"
}

check_qemu() {
  command -v qemu-system-x86_64 >/dev/null 2>&1
}

check_ovmf() {
  local code vars bios
  for code in /usr/share/OVMF/OVMF_CODE.fd /usr/share/edk2/x64/OVMF_CODE.fd /usr/share/qemu/OVMF_CODE.fd; do
    vars="${code%CODE.fd}VARS.fd"
    if [[ -f "$code" && -f "$vars" ]]; then
      return 0
    fi
  done
  for bios in /usr/share/OVMF/OVMF.fd /usr/share/qemu/OVMF.fd; do
    if [[ -f "$bios" ]]; then
      return 0
    fi
  done
  return 1
}

cmd=${1:-}
case "$cmd" in
  rust-target)
    check_rust_target
    ;;
  qemu)
    check_qemu
    ;;
  ovmf)
    check_ovmf
    ;;
  *)
    echo "usage: $0 {rust-target|qemu|ovmf}" >&2
    exit 2
    ;;
esac
