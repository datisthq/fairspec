from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.actions.path.general import get_file_name_slug

from .data import get_data_first_path

if TYPE_CHECKING:
    from fairspec_metadata.models.resource import Resource


def infer_resource_name(resource: Resource, *, resource_number: int | None = None) -> str:
    first_path = get_data_first_path(resource)

    if first_path:
        name = get_file_name_slug(first_path)
        if name:
            return name

    return f"resource{resource_number if resource_number is not None else ''}"
