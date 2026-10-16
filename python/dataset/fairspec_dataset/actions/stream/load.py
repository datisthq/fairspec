from __future__ import annotations

import urllib.request
from io import BytesIO
from typing import BinaryIO

from fairspec_metadata import ResourceDataPath, get_is_remote_path


def load_file_stream(
    data_path: ResourceDataPath,
    *,
    index: int = 0,
    max_bytes: int | None = None,
) -> BinaryIO:
    paths = data_path if isinstance(data_path, list) else [data_path]

    if index >= len(paths) or index < 0:
        raise ValueError(
            f"Cannot stream resource {paths[index] if index < len(paths) else None} at index {index}"
        )

    path = paths[index]
    is_remote = get_is_remote_path(path)

    if is_remote:
        return _load_remote_file_stream(path, max_bytes=max_bytes)

    return _load_local_file_stream(path, max_bytes=max_bytes)


def _load_local_file_stream(
    path: str,
    *,
    max_bytes: int | None = None,
) -> BinaryIO:
    if max_bytes is not None:
        with open(path, "rb") as f:
            data = f.read(max_bytes)
        return BytesIO(data)

    return open(path, "rb")


def _load_remote_file_stream(
    path: str,
    *,
    max_bytes: int | None = None,
) -> BinaryIO:
    with urllib.request.urlopen(path) as response:
        if max_bytes is not None:
            data = response.read(max_bytes)
        else:
            data = response.read()

    return BytesIO(data)
