from __future__ import annotations

import os

from .stringify import stringify_descriptor
from fairspec_metadata.models.descriptor import Descriptor


def save_descriptor(
    descriptor: Descriptor,
    *,
    path: str,
    overwrite: bool = False,
) -> None:
    text = stringify_descriptor(descriptor)
    os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
    mode = "w" if overwrite else "x"
    with open(path, mode, encoding="utf-8") as file:
        file.write(text)
