#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")" && pwd)"
os="$(uname -s)"

case "$os" in
    Linux|Darwin) ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "qs: on Windows run: powershell -File install.ps1" >&2
        exit 1
        ;;
    *)
        echo "qs: unsupported system $os" >&2
        exit 1
        ;;
esac

echo "qs: compiling..."
cargo build --release --manifest-path "$root/Cargo.toml" --target-dir "$root/target"

dest="${HOME:?HOME is not set}/.local/bin"
mkdir -p "$dest"
install -m 755 "$root/target/release/quicksearch" "$dest/qs"

echo "qs: installed $dest/qs"
case ":$PATH:" in
    *":$dest:"*) ;;
    *)
        echo "qs: add this to your shell config:"
        echo "  export PATH=\"$dest:\$PATH\""
        ;;
esac
