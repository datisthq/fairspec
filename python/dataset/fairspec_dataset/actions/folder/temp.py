from __future__ import annotations

import atexit
import shutil
import tempfile


def get_temp_folder_path(*, persist: bool = False) -> str:
    path = tempfile.mkdtemp()

    if not persist:
        atexit.register(_cleanup_dir, path)

    return path


def _cleanup_dir(path: str) -> None:
    try:
        shutil.rmtree(path)
    except OSError:
        pass
