from __future__ import annotations

from io import BytesIO
from typing import BinaryIO


def concat_file_streams(streams: list[BinaryIO]) -> BinaryIO:
    parts: list[bytes] = []
    for stream in streams:
        parts.append(stream.read())
    return BytesIO(b"".join(parts))
