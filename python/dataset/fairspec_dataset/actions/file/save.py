from __future__ import annotations

from io import BytesIO

from fairspec_dataset.actions.stream.save import save_file_stream


def save_file(path: str, data: bytes, *, overwrite: bool = False) -> None:
    save_file_stream(BytesIO(data), path=path, overwrite=overwrite)
