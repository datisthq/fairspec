from __future__ import annotations

from fairspec_dataset.actions.stream.load import load_file_stream


def load_file(path: str, *, max_bytes: int | None = None) -> bytes:
    stream = load_file_stream(path, max_bytes=max_bytes)
    return stream.read()
