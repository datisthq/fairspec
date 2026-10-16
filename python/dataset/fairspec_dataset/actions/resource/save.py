from __future__ import annotations

import re
from typing import TYPE_CHECKING, Callable

from fairspec_metadata import (
    denormalize_path,
    get_file_name,
    get_is_remote_path,
)

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor
    from fairspec_metadata import Resource


class SaveFileProps:
    def __init__(
        self,
        *,
        property_name: str,
        property_index: int,
        normalized_path: str,
        denormalized_path: str,
    ) -> None:
        self.property_name = property_name
        self.property_index = property_index
        self.normalized_path = normalized_path
        self.denormalized_path = denormalized_path


SaveFileCallback = Callable[[SaveFileProps], str]


def save_resource_files(
    resource: Resource,
    *,
    save_file: SaveFileCallback,
    basepath: str | None = None,
    with_remote: bool = False,
    without_folders: bool = False,
) -> Descriptor:
    resource = resource.model_copy(deep=True)
    dedup_indexes: dict[str, int] = {}

    def _save_file(path: str, name: str, index: int) -> str:
        is_remote = get_is_remote_path(path)

        denormalized_path = denormalize_path(path, basepath=basepath)
        normalized_path = path

        if is_remote:
            if not with_remote:
                return path
            filename = get_file_name(path)
            if not filename:
                return path
            denormalized_path = filename
        elif without_folders:
            denormalized_path = denormalized_path.replace("/", "-")

        dedup_index = dedup_indexes.get(denormalized_path, 0)
        dedup_indexes[denormalized_path] = dedup_index + 1

        if dedup_index:
            denormalized_path = re.sub(
                r"^(.*?)([^/]+?)(\.[^/]+(?:\.[^/]+)*)$",
                rf"\1\2-{dedup_index}\3",
                denormalized_path,
            )

        return save_file(
            SaveFileProps(
                property_name=name,
                property_index=index,
                normalized_path=normalized_path,
                denormalized_path=denormalized_path,
            )
        )

    if isinstance(resource.data, str):
        resource.data = _save_file(resource.data, "data", 0)

    if isinstance(resource.data, list):
        for i, item in enumerate(resource.data):
            if isinstance(item, str):
                resource.data[i] = _save_file(item, "data", i)

    for name in ("dataSchema", "tableSchema"):
        prop = getattr(resource, name, None)
        if isinstance(prop, str):
            setattr(resource, name, _save_file(prop, name, 0))

    return resource.model_dump(by_alias=True, exclude_none=True)
