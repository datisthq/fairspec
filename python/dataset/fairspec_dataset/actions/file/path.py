from __future__ import annotations

import os


def get_is_local_path_exist(path: str) -> bool:
    return os.path.exists(path)


def assert_local_path_vacant(path: str) -> None:
    if os.path.exists(path):
        raise FileExistsError(f'Path "{path}" already exists')
