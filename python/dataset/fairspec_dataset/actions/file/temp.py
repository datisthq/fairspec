from __future__ import annotations

import atexit
import os
import tempfile


def write_temp_file(
    content: str | bytes,
    *,
    persist: bool = False,
    filename: str | None = None,
    format: str | None = None,
) -> str:
    path = get_temp_file_path(persist=persist, filename=filename, format=format)
    mode = "wb" if isinstance(content, bytes) else "w"
    encoding = None if isinstance(content, bytes) else "utf-8"
    newline = None if isinstance(content, bytes) else ""
    with open(path, mode, encoding=encoding, newline=newline) as f:
        f.write(content)
    return path


def get_temp_file_path(
    *,
    persist: bool = False,
    filename: str | None = None,
    format: str | None = None,
) -> str:
    if filename:
        dir_path = tempfile.mkdtemp()
        path = os.path.join(dir_path, filename)
    else:
        suffix = f".{format}" if format else ""
        fd, path = tempfile.mkstemp(suffix=suffix)
        os.close(fd)
        os.unlink(path)

    if not persist:
        atexit.register(_cleanup_file, path)

    return path


def _cleanup_file(path: str) -> None:
    try:
        os.unlink(path)
    except OSError:
        pass
