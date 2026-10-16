from __future__ import annotations

from fairspec_dataset.actions.stream.load import load_file_stream
from fairspec_dataset.actions.stream.save import save_file_stream


def copy_file(
    *,
    source_path: str,
    target_path: str,
    max_bytes: int | None = None,
) -> None:
    stream = load_file_stream(source_path, max_bytes=max_bytes)
    save_file_stream(stream, path=target_path)
