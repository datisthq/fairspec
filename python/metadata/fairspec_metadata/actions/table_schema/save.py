from __future__ import annotations

from fairspec_metadata.actions.descriptor.save import save_descriptor
from fairspec_metadata.models.table_schema import TableSchema
from fairspec_metadata.settings import FAIRSPEC_VERSION


def save_table_schema(
    table_schema: TableSchema,
    *,
    path: str,
    overwrite: bool = False,
) -> None:
    descriptor = table_schema.model_dump(by_alias=True, exclude_none=True)

    if "$schema" not in descriptor:
        descriptor["$schema"] = (
            f"https://fairspec.org/profiles/{FAIRSPEC_VERSION}/table-schema.json"
        )

    save_descriptor(descriptor, path=path, overwrite=overwrite)
