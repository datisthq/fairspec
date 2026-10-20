from __future__ import annotations

import codecs

from fairspec_dataset.models.file import FilePreview

from .infer import _is_binary
from .load import load_file


def preview_file(
    path: str,
    *,
    max_bytes: int | None = None,
    max_lines: int | None = None,
) -> FilePreview:
    """Read the raw source of a local or remote file as UTF-8 text, up to the first
    `max_lines` lines (default 20) or `max_bytes` bytes (default 10,000), whichever
    comes first."""
    max_bytes = 10_000 if max_bytes is None else max_bytes
    max_lines = 20 if max_lines is None else max_lines
    _assert_positive_integer("max_bytes", max_bytes)
    _assert_positive_integer("max_lines", max_lines)

    # one extra byte tells a file of exactly max_bytes apart from a longer one
    buffer = load_file(path, max_bytes=max_bytes + 1)
    sample = buffer[:max_bytes]
    truncated = len(buffer) > max_bytes

    if _is_binary(sample):
        raise ValueError("Cannot preview a binary file")

    # a non-final decode drops a multi-byte character cut by the byte limit
    decoder = codecs.getincrementaldecoder("utf-8")("replace")
    text = decoder.decode(sample, final=not truncated)

    end = -1
    for _ in range(max_lines):
        end = text.find("\n", end + 1)
        if end == -1:
            break

    if end != -1 and end + 1 < len(text):
        text = text[: end + 1]
        truncated = True

    return FilePreview(text=text, truncated=truncated)


def _assert_positive_integer(name: str, value: int) -> None:
    if isinstance(value, bool) or not isinstance(value, int) or value <= 0:
        raise ValueError(f"{name} must be a positive integer")
