from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from fairspec_metadata.models.data import ResourceDataValue
    from fairspec_metadata.models.resource import Resource


def get_data_path(resource: Resource) -> str | list[str] | None:
    data = resource.data

    if isinstance(data, str):
        return data

    if isinstance(data, list) and all(isinstance(item, str) for item in data):
        return data

    return None


def get_data_value(resource: Resource) -> ResourceDataValue | None:
    data_path = get_data_path(resource)

    if not data_path:
        return resource.data

    return None


def get_data_records(resource: Resource) -> list[dict] | None:
    data_value = get_data_value(resource)
    if not data_value:
        return None

    return data_value if isinstance(data_value, list) else None


def get_data_paths(resource: Resource) -> list[str]:
    data_path = get_data_path(resource)
    if not data_path:
        return []
    return data_path if isinstance(data_path, list) else [data_path]


def get_data_first_path(resource: Resource) -> str | None:
    data_path = get_data_path(resource)
    if not data_path:
        return None
    return data_path[0] if isinstance(data_path, list) else data_path
