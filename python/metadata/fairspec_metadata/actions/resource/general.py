from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.actions.path.general import get_is_remote_path

from .data import get_data_path

if TYPE_CHECKING:
    from fairspec_metadata.models.resource import Resource


def get_is_remote_resource(resource: Resource) -> bool:
    data_path = get_data_path(resource)
    if not data_path:
        return False

    paths = data_path if isinstance(data_path, list) else [data_path]
    return any(get_is_remote_path(path) for path in paths)
