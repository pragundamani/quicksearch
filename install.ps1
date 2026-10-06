$ErrorActionPreference = "Stop"

$root = Split-Path -Parent $MyInvocation.MyCommand.Path
Write-Host "qs: compiling..."
cargo build --release --manifest-path (Join-Path $root "Cargo.toml") --target-dir (Join-Path $root "target")

$dest = Join-Path $env:LOCALAPPDATA "quicksearch\bin"
New-Item -ItemType Directory -Force -Path $dest | Out-Null
Copy-Item (Join-Path $root "target\release\quicksearch.exe") (Join-Path $dest "qs.exe") -Force

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if (-not $userPath) {
    $userPath = ""
}
$parts = $userPath.Split(";") | Where-Object { $_ -and ($_.TrimEnd("\") -ne $dest.TrimEnd("\")) }
$updated = (@($dest) + $parts) -join ";"
[Environment]::SetEnvironmentVariable("Path", $updated, "User")

Write-Host "qs: installed $(Join-Path $dest 'qs.exe')"
Write-Host "qs: open a new terminal so PATH picks up $dest"
