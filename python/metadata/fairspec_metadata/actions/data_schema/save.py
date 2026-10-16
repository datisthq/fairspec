from __future__ import annotations

from fairspec_metadata.actions.descriptor.copy import copy_descriptor
from fairspec_metadata.actions.descriptor.save import save_descriptor
from fairspec_metadata.models.data_schema import DataSchema
from fairspec_metadata.settings import FAIRSPEC_VERSION


def save_data_schema(
    data_schema: DataSchema,
    *,
    path: str,
    overwrite: bool = False,
) -> None:
    descriptor = copy_descriptor(data_schema)

    if "$schema" not in descriptor:
        descriptor["$schema"] = (
            f"https://fairspec.org/profiles/{FAIRSPEC_VERSION}/data-schema.json"
        )

    save_descriptor(descriptor, path=path, overwrite=overwrite)
