from __future__ import annotations

import hashlib
import os
from typing import TYPE_CHECKING

from charset_normalizer import from_bytes

from fairspec_metadata import Integrity, get_data_first_path
from fairspec_metadata import IntegrityType

from fairspec_dataset.actions.file.load import load_file
from fairspec_dataset.actions.file.prefetch import prefetch_files
from fairspec_dataset.actions.stream.concat import concat_file_streams
from fairspec_dataset.actions.stream.load import load_file_stream
from fairspec_dataset.helpers.concurrency import run_concurrent_tasks

if TYPE_CHECKING:
    from fairspec_metadata import Resource


def infer_textual(
    resource: Resource,
    *,
    sample_bytes: int = 10_000,
) -> bool:
    first_path = get_data_first_path(resource)
    if not first_path:
        return False

    buffer = load_file(first_path, max_bytes=sample_bytes)

    if len(buffer) == 0:
        return True

    if _is_binary(buffer):
        return False

    try:
        buffer.decode("utf-8")
        return True
    except UnicodeDecodeError:
        pass

    results = from_bytes(buffer)
    best = results.best()
    if best is not None:
        encoding = best.encoding.lower()
        return encoding in ("utf-8", "ascii")

    return False


def infer_integrity(
    resource: Resource,
    *,
    hash_type: str = "sha256",
    concurrency: int | None = None,
) -> Integrity | None:
    hash_value = infer_hash(resource, hash_type=hash_type, concurrency=concurrency)

    if not hash_value:
        return None

    return Integrity(type=IntegrityType(hash_type), hash=hash_value)


def infer_hash(
    resource: Resource,
    *,
    hash_type: str = "sha256",
    concurrency: int | None = None,
) -> str:
    local_paths = prefetch_files(resource, concurrency=concurrency)

    if not local_paths:
        return ""

    streams = run_concurrent_tasks(load_file_stream, local_paths, concurrency=concurrency)
    stream = concat_file_streams(streams)

    h = hashlib.new(hash_type)
    h.update(stream.read())
    return h.hexdigest()


def infer_bytes(resource: Resource, *, concurrency: int | None = None) -> int:
    local_paths = prefetch_files(resource, concurrency=concurrency)

    total = 0
    for local_path in local_paths:
        total += os.stat(local_path).st_size

    return total


def _is_binary(data: bytes) -> bool:
    control_chars = set(range(0, 8)) | set(range(14, 32))
    return any(byte in control_chars for byte in data)
