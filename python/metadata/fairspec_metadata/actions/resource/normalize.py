from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.actions.path.normalize import normalize_path

if TYPE_CHECKING:
    from fairspec_metadata.models.descriptor import Descriptor
    from fairspec_metadata.models.resource import Resource


def normalize_resource(resource: Resource, *, basepath: str | None = None) -> Descriptor:
    resource = resource.model_copy(deep=True)

    if isinstance(resource.data, str):
        resource.data = normalize_path(resource.data, basepath=basepath)

    if isinstance(resource.data, list):
        for index, path in enumerate(resource.data):
            if isinstance(path, str):
                resource.data[index] = normalize_path(path, basepath=basepath)

    for name in ("fileDialect", "dataSchema", "tableSchema"):
        value = getattr(resource, name, None)
        if isinstance(value, str):
            setattr(resource, name, normalize_path(value, basepath=basepath))

    return resource.model_dump(by_alias=True, exclude_none=True)
