#!/usr/bin/env bash
set -euo pipefail

release=0
if [[ "${1:-}" == "--release" ]]; then
    release=1
elif [[ -n "${1:-}" ]]; then
    echo "qs: unknown option $1" >&2
    echo "qs: usage: install.sh [--release]" >&2
    exit 2
fi

os="$(uname -s)"

case "$os" in
    Linux|Darwin) ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "qs: on Windows run: powershell -File install.ps1 -Release" >&2
        exit 1
        ;;
    *)
        echo "qs: unsupported system $os" >&2
        exit 1
        ;;
esac

install_unix_bin() {
    local src="$1"
    local dest="${HOME:?HOME is not set}/.local/bin"
    mkdir -p "$dest"
    install -m 755 "$src" "$dest/qs"
    echo "qs: installed $dest/qs"
    case ":$PATH:" in
        *":$dest:"*) ;;
        *)
            echo "qs: add this to your shell config:"
            echo "  export PATH=\"$dest:\$PATH\""
            ;;
    esac
}

if [[ "$release" == 1 ]]; then
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)
            echo "qs: unsupported architecture $(uname -m)" >&2
            exit 1
            ;;
    esac
    case "$os" in
        Linux) target="${arch}-unknown-linux-gnu" ;;
        Darwin) target="${arch}-apple-darwin" ;;
    esac
    asset="qs-${target}.tar.gz"
    url="https://github.com/pragundamani/quicksearch/releases/latest/download/${asset}"
    tmp="$(mktemp -d)"
    trap 'rm -rf "$tmp"' EXIT
    echo "qs: downloading ${asset}"
    curl -fsSL -o "$tmp/$asset" -A "qs-install" "$url"
    tar -xzf "$tmp/$asset" -C "$tmp"
    install_unix_bin "$tmp/qs"
    exit 0
fi

root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
echo "qs: compiling..."
cargo build --release --manifest-path "$root/Cargo.toml" --target-dir "$root/target"
install_unix_bin "$root/target/release/quicksearch"
