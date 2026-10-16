from __future__ import annotations

import os


def create_folder(path: str) -> None:
    os.makedirs(path, exist_ok=True)
