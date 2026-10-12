#!/usr/bin/env python3
"""Create and verify the deterministic Windows release ZIP."""

from __future__ import annotations

import argparse
import os
import stat
import tempfile
import zlib
from pathlib import Path, PurePosixPath
from zipfile import BadZipFile, ZIP_DEFLATED, ZipFile, ZipInfo

MEMBER_NAME = "qs.exe"
FIXED_TIMESTAMP = (1980, 1, 1, 0, 0, 0)
FILE_MODE = stat.S_IFREG | 0o755
MAX_SOURCE_SIZE = 128 * 1024 * 1024
ZIP_MAGIC = b"PK\x03\x04"


def fail(message: str) -> None:
    raise ValueError(message)


def read_source(source: Path) -> bytes:
    try:
        source_stat = source.lstat()
    except OSError as error:
        fail(f"cannot stat source {source}: {error}")

    if not stat.S_ISREG(source_stat.st_mode):
        fail(f"source is not a regular file: {source}")
    if source_stat.st_size == 0:
        fail("source must not be empty")
    if source_stat.st_size > MAX_SOURCE_SIZE:
        fail(f"source exceeds the {MAX_SOURCE_SIZE}-byte size limit")

    try:
        with source.open("rb") as source_file:
            opened_stat = os.fstat(source_file.fileno())
            if not stat.S_ISREG(opened_stat.st_mode):
                fail(f"opened source is not a regular file: {source}")
            if (opened_stat.st_dev, opened_stat.st_ino) != (
                source_stat.st_dev,
                source_stat.st_ino,
            ):
                fail("source changed while it was being opened")
            if opened_stat.st_size != source_stat.st_size:
                fail("source size changed while it was being opened")
            source_bytes = source_file.read(MAX_SOURCE_SIZE + 1)
    except OSError as error:
        fail(f"cannot read source {source}: {error}")

    if len(source_bytes) != source_stat.st_size:
        fail("source size changed while it was being read")
    return source_bytes


def validate_archive(archive_path: Path, source_bytes: bytes) -> None:
    with archive_path.open("rb") as archive_file:
        if archive_file.read(4) != ZIP_MAGIC:
            fail("archive does not start with the ZIP local-file-header magic")

    try:
        with ZipFile(archive_path, "r") as archive:
            infos = archive.infolist()
            if len(infos) != 1:
                fail(f"archive must contain exactly one member, found {len(infos)}")

            info = infos[0]
            member_path = PurePosixPath(info.filename)
            if (
                info.filename != MEMBER_NAME
                or member_path.is_absolute()
                or len(member_path.parts) != 1
            ):
                fail(f"unsafe or unexpected archive member: {info.filename!r}")
            if info.is_dir():
                fail("archive member must be a regular file")
            if info.create_system != 3:
                fail("archive member must use Unix mode metadata")

            archived_mode = info.external_attr >> 16
            if not stat.S_ISREG(archived_mode):
                fail("archive member type is not a regular file")
            if stat.S_IMODE(archived_mode) != 0o755:
                fail(
                    f"archive member mode is {stat.S_IMODE(archived_mode):04o}, "
                    "expected 0755"
                )
            if info.date_time != FIXED_TIMESTAMP:
                fail(f"archive member timestamp is {info.date_time!r}")
            if info.compress_type != ZIP_DEFLATED:
                fail("archive member does not use DEFLATE")
            if info.file_size != len(source_bytes):
                fail("archive member size does not match the source")

            bad_member = archive.testzip()
            if bad_member is not None:
                fail(f"CRC check failed for {bad_member!r}")

            expected_crc = zlib.crc32(source_bytes) & 0xFFFFFFFF
            if info.CRC != expected_crc:
                fail("archive member CRC does not match the source")
            if archive.read(info) != source_bytes:
                fail("archive member bytes do not match the source")
    except BadZipFile as error:
        fail(f"invalid ZIP archive: {error}")


def cleanup_owned_temp(temp_path: Path, identity: tuple[int, int]) -> None:
    try:
        current_stat = temp_path.lstat()
    except FileNotFoundError:
        return
    if (
        stat.S_ISREG(current_stat.st_mode)
        and (current_stat.st_dev, current_stat.st_ino) == identity
    ):
        temp_path.unlink()


def package(source: Path, output: Path) -> None:
    source_bytes = read_source(source)
    output.parent.mkdir(parents=True, exist_ok=True)

    try:
        output_stat = output.lstat()
    except FileNotFoundError:
        output_stat = None
    except OSError as error:
        fail(f"cannot stat output {output}: {error}")

    if output_stat is not None:
        if stat.S_ISLNK(output_stat.st_mode):
            fail(f"output must not be a symbolic link: {output}")
        if not stat.S_ISREG(output_stat.st_mode):
            fail(f"output is not a regular file: {output}")
        try:
            if source.samefile(output):
                fail("source and output refer to the same file")
        except OSError as error:
            fail(f"cannot compare source and output: {error}")

    info = ZipInfo(MEMBER_NAME, date_time=FIXED_TIMESTAMP)
    info.create_system = 3
    info.compress_type = ZIP_DEFLATED
    info.external_attr = FILE_MODE << 16
    info.internal_attr = 0
    info.extra = b""
    info.comment = b""

    temp_fd, temp_name = tempfile.mkstemp(
        prefix=f".{output.name}.", suffix=".tmp", dir=output.parent
    )
    temp_path = Path(temp_name)
    temp_stat = os.fstat(temp_fd)
    temp_identity = (temp_stat.st_dev, temp_stat.st_ino)
    replaced = False
    try:
        with os.fdopen(temp_fd, "w+b") as temp_file:
            with ZipFile(
                temp_file, "w", compression=ZIP_DEFLATED, compresslevel=9
            ) as archive:
                archive.comment = b""
                archive.writestr(
                    info,
                    source_bytes,
                    compress_type=ZIP_DEFLATED,
                    compresslevel=9,
                )
            temp_file.flush()
            os.fsync(temp_file.fileno())

        validate_archive(temp_path, source_bytes)
        os.replace(temp_path, output)
        replaced = True
    finally:
        if not replaced:
            cleanup_owned_temp(temp_path, temp_identity)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    package(args.source, args.output)
    print(f"created and verified {args.output}")


if __name__ == "__main__":
    main()
