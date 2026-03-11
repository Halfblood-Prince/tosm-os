$ErrorActionPreference = "Stop"

. "$PSScriptRoot\common.ps1"

$repoRoot = Get-RepoRoot
$targetDir = Join-Path $repoRoot "build\cargo-target"
$expected = "tosm-os: kernel entry reached"

& cargo build `
    --manifest-path (Join-Path $repoRoot "boot\uefi\Cargo.toml") `
    --target x86_64-unknown-uefi `
    --target-dir $targetDir

& "$PSScriptRoot\run-qemu.ps1"

$serialLog = Join-Path $repoRoot "build\qemu-serial.log"
if (-not (Test-Path $serialLog)) {
    throw "Smoke test failed: serial log was not produced"
}

$content = Get-Content $serialLog -Raw
if ($content -notmatch [regex]::Escape($expected)) {
    throw "Smoke test failed: expected serial banner '$expected' was not found"
}

Write-Host "smoke ok"
