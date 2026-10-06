#!/usr/bin/env python3
"""Build qs and install it. Works on Linux, macOS, and Windows."""

import os
import shutil
import stat
import subprocess
import sys
from pathlib import Path


def main() -> int:
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
    if sys.platform == "win32":
        return install_windows(root)
    if sys.platform.startswith(("linux", "darwin", "freebsd")):
        return install_unix(root)
    print(f"qs: unsupported system {sys.platform}", file=sys.stderr)
    return 1


def install_unix(root: Path) -> int:
    home = os.environ.get("HOME")
    if not home:
        print("qs: HOME is not set", file=sys.stderr)
        return 1
    dest_dir = Path(home) / ".local" / "bin"
    dest_dir.mkdir(parents=True, exist_ok=True)
    dest = dest_dir / "qs"
    shutil.copy2(root / "target" / "release" / "quicksearch", dest)
    dest.chmod(dest.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
    print(f"qs: installed {dest}")
    path_entries = os.environ.get("PATH", "").split(os.pathsep)
    if str(dest_dir) not in path_entries:
        print("qs: add this to your shell config:")
        print(f'  export PATH="{dest_dir}:$PATH"')
    return 0


def install_windows(root: Path) -> int:
    import winreg

    local = os.environ.get("LOCALAPPDATA")
    if not local:
        print("qs: LOCALAPPDATA is not set", file=sys.stderr)
        return 1
    dest_dir = Path(local) / "quicksearch" / "bin"
    dest_dir.mkdir(parents=True, exist_ok=True)
    dest = dest_dir / "qs.exe"
    shutil.copy2(root / "target" / "release" / "quicksearch.exe", dest)
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
        parts = [part for part in parts if os.path.normcase(os.path.normpath(part)) != os.path.normcase(os.path.normpath(dest))]
        winreg.SetValueEx(key, "Path", 0, kind, ";".join([dest, *parts]))


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except subprocess.CalledProcessError as err:
        raise SystemExit(err.returncode) from err
