from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import get_data_paths, get_is_remote_path

from fairspec_dataset.actions.file.copy import copy_file
from fairspec_dataset.actions.file.temp import get_temp_file_path
from fairspec_dataset.helpers.concurrency import run_concurrent_tasks

if TYPE_CHECKING:
    from fairspec_metadata import Resource


def prefetch_files(
    resource: Resource,
    *,
    max_bytes: int | None = None,
    concurrency: int | None = None,
) -> list[str]:
    paths = get_data_paths(resource)
    if not paths:
        return []

    def prefetch(path: str) -> str:
        return prefetch_file(path, max_bytes=max_bytes)

    return run_concurrent_tasks(prefetch, paths, concurrency=concurrency)


def prefetch_file(
    path: str,
    *,
    max_bytes: int | None = None,
) -> str:
    if not get_is_remote_path(path):
        return path

    new_path = get_temp_file_path()
    copy_file(source_path=path, target_path=new_path, max_bytes=max_bytes)
    return new_path
