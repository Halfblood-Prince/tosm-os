$ErrorActionPreference = "Stop"

function Get-RepoRoot {
    return Split-Path -Parent $PSScriptRoot
}

function Get-QemuPath {
    $candidates = @(
        "qemu-system-x86_64.exe",
        "C:\Program Files\qemu\qemu-system-x86_64.exe",
        "C:\msys64\mingw64\bin\qemu-system-x86_64.exe"
    )

    foreach ($candidate in $candidates) {
        if (Test-Path $candidate) {
            return (Resolve-Path $candidate).Path
        }
    }

    $command = Get-Command qemu-system-x86_64.exe -ErrorAction SilentlyContinue
    if ($command) {
        return $command.Source
    }

    throw "Unable to locate qemu-system-x86_64.exe"
}

function Get-OvmfConfig {
    $pairs = @(
        @{
            Code = "C:\Program Files\qemu\share\edk2-x86_64-code.fd"
            Vars = "C:\Program Files\qemu\share\edk2-i386-vars.fd"
        },
        @{
            Code = "C:\Program Files\qemu\share\OVMF_CODE.fd"
            Vars = "C:\Program Files\qemu\share\OVMF_VARS.fd"
        }
    )

    foreach ($pair in $pairs) {
        if ((Test-Path $pair.Code) -and (Test-Path $pair.Vars)) {
            return $pair
        }
    }

    $singleFile = @(
        "C:\Program Files\qemu\share\OVMF.fd"
    )

    foreach ($candidate in $singleFile) {
        if (Test-Path $candidate) {
            return @{
                Bios = $candidate
            }
        }
    }

    throw "Unable to locate OVMF firmware"
}

function Get-BootEfiPath {
    param(
        [string]$RepoRoot
    )

    return Join-Path $RepoRoot "build\cargo-target\x86_64-unknown-uefi\debug\tosm-uefi.efi"
}

function New-EspLayout {
    param(
        [string]$RepoRoot
    )

    $espRoot = Join-Path $RepoRoot "build\esp"
    $bootDir = Join-Path $espRoot "EFI\BOOT"
    $bootFile = Join-Path $bootDir "BOOTX64.EFI"

    New-Item -ItemType Directory -Force -Path $bootDir | Out-Null
    Copy-Item -Path (Get-BootEfiPath -RepoRoot $RepoRoot) -Destination $bootFile -Force

    return @{
        Root = $espRoot
        BootFile = $bootFile
    }
}
