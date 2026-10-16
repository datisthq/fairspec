from __future__ import annotations

import os
from typing import TYPE_CHECKING

from fairspec_metadata import get_basepath, get_data_paths, get_is_remote_path
from fairspec_metadata.actions.path.general import safe_relpath

if TYPE_CHECKING:
    from fairspec_metadata import Dataset


def get_dataset_basepath(dataset: Dataset) -> str | None:
    paths: list[str] = []

    for resource in dataset.resources or []:
        resource_paths = get_data_paths(resource)
        paths.extend(resource_paths)

    return get_common_local_basepath(paths)


def get_common_local_basepath(paths: list[str]) -> str | None:
    absolute_basepaths = [
        os.path.abspath(get_basepath(path))
        for path in paths
        if not get_is_remote_path(path)
    ]

    if not absolute_basepaths:
        return None

    segment_table = [
        [segment or "/" for segment in path.split(os.sep)] for path in absolute_basepaths
    ]

    column = 0
    segments: list[str] = []

    while True:
        segment_column = [
            row[column] if column < len(row) else None for row in segment_table
        ]
        unique_segments = set(segment_column)

        if len(unique_segments) != 1:
            break
        if segment_column[0] is None:
            break

        column += 1
        segments.append(segment_column[0])

    if not segments:
        raise ValueError("Cannot find common basepath")

    if segments[0].endswith(":"):
        segments[0] += os.sep

    basepath = safe_relpath(os.path.join(*segments))
    return "" if basepath == "." else basepath
