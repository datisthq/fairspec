from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.actions.path.denormalize import denormalize_path

if TYPE_CHECKING:
    from fairspec_metadata.models.descriptor import Descriptor
    from fairspec_metadata.models.resource import Resource


def denormalize_resource(
    resource: Resource, *, basepath: str | None = None
) -> Descriptor:
    resource = resource.model_copy(deep=True)

    if isinstance(resource.data, str):
        resource.data = denormalize_path(resource.data, basepath=basepath)

    if isinstance(resource.data, list):
        for index, path in enumerate(resource.data):
            if isinstance(path, str):
                resource.data[index] = denormalize_path(path, basepath=basepath)

    for name in ("fileDialect", "dataSchema", "tableSchema"):
        value = getattr(resource, name, None)
        if isinstance(value, str):
            setattr(resource, name, denormalize_path(value, basepath=basepath))

    return resource.model_dump(by_alias=True, exclude_none=True)
