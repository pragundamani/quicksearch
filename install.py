#!/usr/bin/env python3
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

"""Build qs and install it, or download a release binary."""

import os
import platform
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
import urllib.request
import zipfile
from pathlib import Path

RELEASE = "https://github.com/pragundamani/quicksearch/releases/latest/download"


def main() -> int:
    args = sys.argv[1:]
    if any(arg != "--release" for arg in args):
        print("qs: usage: install.py [--release]", file=sys.stderr)
        return 2
    release = "--release" in args
    if release:
        binary = download_release()
    else:
        root = Path(__file__).resolve().parent
        print("qs: compiling...")
        subprocess.run(
            [
                "cargo",
                "build",
                "--release",
                "--manifest-path",
                str(root / "Cargo.toml"),
                "--target-dir",
                str(root / "target"),
            ],
            check=True,
        )
        name = "qs.exe" if sys.platform == "win32" else "qs"
        binary = root / "target" / "release" / name
    if sys.platform == "win32":
        return install_windows(binary)
    if sys.platform.startswith(("linux", "darwin", "freebsd")):
        return install_unix(binary)
    print(f"qs: unsupported system {sys.platform}", file=sys.stderr)
    return 1


def download_release() -> Path:
    asset = release_asset()
    url = f"{RELEASE}/{asset}"
    print(f"qs: downloading {asset}")
    tmp = Path(tempfile.mkdtemp(prefix="qs-install-"))
    archive = tmp / asset
    request = urllib.request.Request(url, headers={"User-Agent": "qs-install"})
    with urllib.request.urlopen(request) as response, archive.open("wb") as out:
        shutil.copyfileobj(response, out)
    if asset.endswith(".zip"):
        with zipfile.ZipFile(archive) as zipped:
            zipped.extractall(tmp)
        return tmp / "qs.exe"
    with tarfile.open(archive, "r:gz") as packed:
        packed.extractall(tmp)
    return tmp / "qs"


def release_asset() -> str:
    machine = platform.machine().lower()
    arch = {"x86_64": "x86_64", "amd64": "x86_64", "aarch64": "aarch64", "arm64": "aarch64"}.get(machine)
    if arch is None:
        raise SystemExit(f"qs: unsupported architecture {platform.machine()}")
    if sys.platform == "win32":
        return f"qs-{arch}-pc-windows-msvc.zip"
    if sys.platform == "darwin":
        return f"qs-{arch}-apple-darwin.tar.gz"
    if sys.platform.startswith("linux"):
        return f"qs-{arch}-unknown-linux-gnu.tar.gz"
    raise SystemExit(f"qs: unsupported system {sys.platform}")


def install_unix(binary: Path) -> int:
    home = os.environ.get("HOME")
    if not home:
        print("qs: HOME is not set", file=sys.stderr)
        return 1
    dest_dir = Path(home) / ".local" / "bin"
    dest_dir.mkdir(parents=True, exist_ok=True)
    dest = dest_dir / "qs"
    shutil.copy2(binary, dest)
    dest.chmod(dest.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    print(f"qs: installed {dest}")
    path_entries = os.environ.get("PATH", "").split(os.pathsep)
    if str(dest_dir) not in path_entries:
        print("qs: add this to your shell config:")
        print(f'  export PATH="{dest_dir}:$PATH"')
    return 0


def install_windows(binary: Path) -> int:
    import winreg

    local = os.environ.get("LOCALAPPDATA")
    if not local:
        print("qs: LOCALAPPDATA is not set", file=sys.stderr)
        return 1
    dest_dir = Path(local) / "quicksearch" / "bin"
    dest_dir.mkdir(parents=True, exist_ok=True)
    dest = dest_dir / "qs.exe"
    shutil.copy2(binary, dest)
    prepend_user_path(winreg, dest_dir)
    print(f"qs: installed {dest}")
    print(f"qs: open a new terminal so PATH picks up {dest_dir}")
    return 0


def prepend_user_path(winreg, dest_dir: Path) -> None:
    with winreg.OpenKey(winreg.HKEY_CURRENT_USER, "Environment", 0, winreg.KEY_READ | winreg.KEY_SET_VALUE) as key:
        try:
            current, kind = winreg.QueryValueEx(key, "Path")
        except FileNotFoundError:
            current, kind = "", winreg.REG_EXPAND_SZ
        parts = [part for part in current.split(";") if part]
        dest = str(dest_dir)
        wanted = os.path.normcase(os.path.normpath(dest))
        parts = [part for part in parts if os.path.normcase(os.path.normpath(part)) != wanted]
        winreg.SetValueEx(key, "Path", 0, kind, ";".join([dest, *parts]))


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except subprocess.CalledProcessError as err:
        raise SystemExit(err.returncode) from err
