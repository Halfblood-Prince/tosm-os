#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/common.sh"

ROOT=$(repo_root)
QEMU=$(qemu_bin)
SERIAL_LOG="$ROOT/build/qemu-serial.log"
VARS_COPY="$ROOT/build/OVMF_VARS.fd"

prepare_esp "$ROOT"
rm -f "$SERIAL_LOG"

mapfile -t OVMF < <(find_ovmf)

ARGS=(
  -nodefaults
  -machine q35
  -m 256M
  -display none
  -serial "file:build/qemu-serial.log"
  -drive "format=raw,file=fat:rw:build/esp"
)

if [[ "${OVMF[0]}" == bios=* ]]; then
  ARGS+=( -bios "${OVMF[0]#bios=}" )
else
  CODE="${OVMF[0]#code=}"
  VARS="${OVMF[1]#vars=}"
  cp "$VARS" "$VARS_COPY"
  ARGS+=(
    -drive "if=pflash,format=raw,readonly=on,file=$CODE"
    -drive "if=pflash,format=raw,file=$VARS_COPY"
  )
fi

(
  cd "$ROOT"
  "$QEMU" "${ARGS[@]}"
)
