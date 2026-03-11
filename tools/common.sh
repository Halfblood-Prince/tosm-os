#!/usr/bin/env bash
set -euo pipefail

repo_root() {
  cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd
}

qemu_bin() {
  if command -v qemu-system-x86_64 >/dev/null 2>&1; then
    command -v qemu-system-x86_64
    return
  fi
  echo "Unable to locate qemu-system-x86_64" >&2
  return 1
}

find_ovmf() {
  local code vars bios
  for code in \
    /usr/share/OVMF/OVMF_CODE.fd \
    /usr/share/edk2/x64/OVMF_CODE.fd \
    /usr/share/qemu/OVMF_CODE.fd; do
    vars="${code%CODE.fd}VARS.fd"
    if [[ -f "$code" && -f "$vars" ]]; then
      echo "code=$code"
      echo "vars=$vars"
      return
    fi
  done

  for bios in /usr/share/OVMF/OVMF.fd /usr/share/qemu/OVMF.fd; do
    if [[ -f "$bios" ]]; then
      echo "bios=$bios"
      return
    fi
  done

  echo "Unable to locate OVMF firmware" >&2
  return 1
}

boot_efi_path() {
  local root=$1
  echo "$root/build/cargo-target/x86_64-unknown-uefi/debug/tosm-uefi.efi"
}

prepare_esp() {
  local root=$1
  local boot_dir="$root/build/esp/EFI/BOOT"
  mkdir -p "$boot_dir"
  cp "$(boot_efi_path "$root")" "$boot_dir/BOOTX64.EFI"
}
