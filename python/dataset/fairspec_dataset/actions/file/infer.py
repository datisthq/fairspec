from __future__ import annotations

import codecs
import hashlib
import os
from typing import TYPE_CHECKING

from fairspec_metadata import Integrity, get_data_paths
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
    paths = get_data_paths(resource)
    if not paths:
        return False

    return all(_get_is_textual(path, sample_bytes=sample_bytes) for path in paths)


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


def _get_is_textual(path: str, *, sample_bytes: int) -> bool:
    buffer = load_file(path, max_bytes=sample_bytes + 1)
    is_cut = len(buffer) > sample_bytes
    sample = buffer[:sample_bytes]

    if _is_binary(sample):
        return False

    # only a sample cut short may end inside a character; a whole file may not
    try:
        codecs.getincrementaldecoder("utf-8")().decode(sample, final=not is_cut)
        return True
    except UnicodeDecodeError:
        return False


def _is_binary(data: bytes) -> bool:
    control_chars = set(range(0, 8)) | set(range(14, 32))
    return any(byte in control_chars for byte in data)
