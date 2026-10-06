# qs looks things up from the terminal.
# Copyright (C) 2026 Pragun Damani <damanipragun@proton.me>
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

param(
    [switch]$Release
)

$ErrorActionPreference = "Stop"

function Install-QsBinary([string]$Source) {
    $dest = Join-Path $env:LOCALAPPDATA "quicksearch\bin"
    New-Item -ItemType Directory -Force -Path $dest | Out-Null
    Copy-Item $Source (Join-Path $dest "qs.exe") -Force

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if (-not $userPath) {
        $userPath = ""
    }
    $parts = $userPath.Split(";") | Where-Object { $_ -and ($_.TrimEnd("\") -ne $dest.TrimEnd("\")) }
    $updated = (@($dest) + $parts) -join ";"
    [Environment]::SetEnvironmentVariable("Path", $updated, "User")

    Write-Host "qs: installed $(Join-Path $dest 'qs.exe')"
    Write-Host "qs: open a new terminal so PATH picks up $dest"
}

if ($Release) {
    $arch = switch ([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()) {
        "X64" { "x86_64" }
        "Arm64" { "aarch64" }
        default { throw "qs: unsupported architecture $([System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture)" }
    }
    $asset = "qs-$arch-pc-windows-msvc.zip"
    $url = "https://github.com/pragundamani/quicksearch/releases/latest/download/$asset"
    $tmp = Join-Path $env:TEMP ("qs-install-" + [guid]::NewGuid().ToString("n"))
    New-Item -ItemType Directory -Path $tmp | Out-Null
    try {
        Write-Host "qs: downloading $asset"
        Invoke-WebRequest -Uri $url -OutFile (Join-Path $tmp $asset) -UseBasicParsing
        tar -xf (Join-Path $tmp $asset) -C $tmp
        Install-QsBinary (Join-Path $tmp "qs.exe")
    } finally {
        Remove-Item -Recurse -Force $tmp
    }
    exit 0
}

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
Write-Host "qs: compiling..."
cargo build --release --manifest-path (Join-Path $root "Cargo.toml") --target-dir (Join-Path $root "target")
Install-QsBinary (Join-Path $root "target\release\qs.exe")
