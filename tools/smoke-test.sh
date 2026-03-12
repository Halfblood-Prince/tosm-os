#!/usr/bin/env bash
set -euo pipefail

expected_banner='tosm-os: kernel entry reached'
expected_panic='tosm-os: panic in uefi-entry'
expected_entry_done='tosm-os: efi_main completed'
expected_banner_line='tosm-os: kernel entry reached\r\n'
expected_panic_line='tosm-os: panic in uefi-entry\r\n'
expected_entry_done_line='tosm-os: efi_main completed\r\n'

contract_check() {
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

  echo "smoke: serial message contracts present"
}

find_ovmf_code() {
  local candidate
  for candidate in \
    "${OVMF_CODE_PATH:-}" \
    /usr/share/OVMF/OVMF_CODE_4M.fd \
    /usr/share/OVMF/OVMF_CODE.fd \
    /usr/share/edk2/ovmf/OVMF_CODE.fd \
    /usr/share/edk2/ovmf/OVMF_CODE_4M.fd \
    /usr/share/ovmf/OVMF.fd \
    /usr/share/edk2/x64/OVMF_CODE.fd; do
    if [[ -n "${candidate}" && -f "${candidate}" ]]; then
      printf '%s\n' "${candidate}"
      return 0
    fi
  done
  return 1
}

find_ovmf_vars() {
  local candidate
  for candidate in \
    "${OVMF_VARS_PATH:-}" \
    /usr/share/OVMF/OVMF_VARS_4M.fd \
    /usr/share/OVMF/OVMF_VARS.fd \
    /usr/share/edk2/ovmf/OVMF_VARS.fd \
    /usr/share/edk2/ovmf/OVMF_VARS_4M.fd \
    /usr/share/edk2/x64/OVMF_VARS.fd; do
    if [[ -n "${candidate}" && -f "${candidate}" ]]; then
      printf '%s\n' "${candidate}"
      return 0
    fi
  done
  return 1
}


ensure_uefi_target_installed() {
  local target="x86_64-unknown-uefi"

  if ! command -v rustup >/dev/null 2>&1; then
    echo "smoke: rustup unavailable; cannot provision ${target} target"
    return 1
  fi

  if rustup target list --installed | grep --fixed-strings --quiet -- "${target}"; then
    return 0
  fi

  echo "smoke: installing missing Rust target ${target}"
  rustup target add "${target}"
}

run_qemu_smoke() {
  local qemu_bin="${QEMU_BIN:-qemu-system-x86_64}"
  if ! command -v "${qemu_bin}" >/dev/null 2>&1; then
    if [[ "${REQUIRE_QEMU_SMOKE:-0}" -eq 1 ]]; then
      echo "smoke: ${qemu_bin} unavailable but REQUIRE_QEMU_SMOKE=1"
      exit 1
    fi
    echo "smoke: ${qemu_bin} unavailable, skipping QEMU execution"
    return 2
  fi

  local ovmf_code ovmf_vars
  if ! ovmf_code="$(find_ovmf_code)"; then
    if [[ "${REQUIRE_QEMU_SMOKE:-0}" -eq 1 ]]; then
      echo "smoke: OVMF code firmware unavailable but REQUIRE_QEMU_SMOKE=1"
      exit 1
    fi
    echo "smoke: OVMF code firmware unavailable, skipping QEMU execution"
    return 2
  fi
  if ! ovmf_vars="$(find_ovmf_vars)"; then
    if [[ "${REQUIRE_QEMU_SMOKE:-0}" -eq 1 ]]; then
      echo "smoke: OVMF vars firmware unavailable but REQUIRE_QEMU_SMOKE=1"
      exit 1
    fi
    echo "smoke: OVMF vars firmware unavailable, skipping QEMU execution"
    return 2
  fi

  ensure_uefi_target_installed

  cargo build --package uefi-entry --bin bootx64 --target x86_64-unknown-uefi

  local efi_path="target/x86_64-unknown-uefi/debug/bootx64.efi"
  if [[ ! -f "${efi_path}" ]]; then
    echo "smoke: expected EFI image missing at ${efi_path}"
    exit 1
  fi

  SMOKE_RUN_DIR="$(mktemp -d)"
  trap 'rm -rf "${SMOKE_RUN_DIR}"' EXIT
  mkdir -p "${SMOKE_RUN_DIR}/EFI/BOOT"
  cp "${efi_path}" "${SMOKE_RUN_DIR}/EFI/BOOT/BOOTX64.EFI"

  # OVMF variable stores are mutable. Always use a temp copy so each run is deterministic
  # and never mutates global firmware state in CI workers.
  local ovmf_vars_runtime="${SMOKE_RUN_DIR}/OVMF_VARS.fd"
  cp "${ovmf_vars}" "${ovmf_vars_runtime}"

  local serial_log="${SMOKE_RUN_DIR}/serial.log"
  timeout 20s "${qemu_bin}" \
    -nodefaults \
    -nographic \
    -serial file:"${serial_log}" \
    -drive if=pflash,format=raw,readonly=on,file="${ovmf_code}" \
    -drive if=pflash,format=raw,file="${ovmf_vars_runtime}" \
    -drive format=raw,file=fat:rw:"${SMOKE_RUN_DIR}"

  if ! grep --fixed-strings --quiet -- "${expected_banner}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing banner"
    exit 1
  fi

  if ! grep --fixed-strings --quiet -- "${expected_entry_done}" "${serial_log}"; then
    echo "smoke: QEMU serial output missing completion line"
    exit 1
  fi

  echo "smoke: QEMU boot output includes banner and completion line"
}

contract_check
qemu_status=0
run_qemu_smoke || qemu_status=$?
if [[ "${qemu_status}" -ne 0 && "${qemu_status}" -ne 2 ]]; then
  exit "${qemu_status}"
fi
