$ErrorActionPreference = "Stop"

. "$PSScriptRoot\common.ps1"

$repoRoot = Get-RepoRoot
$qemu = Get-QemuPath
$ovmf = Get-OvmfConfig
$esp = New-EspLayout -RepoRoot $repoRoot
$serialLog = Join-Path $repoRoot "build\qemu-serial.log"
$varsCopy = Join-Path $repoRoot "build\OVMF_VARS.fd"

Remove-Item -ErrorAction SilentlyContinue $serialLog

$arguments = @(
    "-nodefaults"
    "-machine", "q35"
    "-m", "256M"
    "-display", "none"
    "-serial", "file:build/qemu-serial.log"
    "-drive", "format=raw,file=fat:rw:build/esp"
)

if ($ovmf.ContainsKey("Bios")) {
    $arguments += @("-bios", $ovmf.Bios)
} else {
    Copy-Item -Path $ovmf.Vars -Destination $varsCopy -Force
    $arguments += @(
        "-drive", "if=pflash,format=raw,readonly=on,file=$($ovmf.Code)"
        "-drive", "if=pflash,format=raw,file=$varsCopy"
    )
}

Push-Location $repoRoot
try {
    & $qemu @arguments
} finally {
    Pop-Location
}
