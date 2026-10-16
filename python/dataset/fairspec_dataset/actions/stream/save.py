from __future__ import annotations

import os
import shutil
from typing import BinaryIO


def save_file_stream(
    stream: BinaryIO,
    *,
    path: str,
    overwrite: bool = False,
) -> None:
    os.makedirs(os.path.dirname(path) or ".", exist_ok=True)

    if not overwrite and os.path.exists(path):
        raise FileExistsError(f'Path "{path}" already exists')

    with open(path, "wb") as f:
        shutil.copyfileobj(stream, f)
