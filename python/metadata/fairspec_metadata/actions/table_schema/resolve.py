from __future__ import annotations

from fairspec_metadata.actions.frictionless.convert import get_is_frictionless_descriptor
from fairspec_metadata.models.table_schema import TableSchema

from .assert_ import assert_table_schema
from .load import load_table_schema


def resolve_table_schema(
    table_schema: TableSchema | str | None = None,
) -> TableSchema | None:
    if table_schema is None:
        return None

    if isinstance(table_schema, str):
        return load_table_schema(table_schema)

    if "fields" in (table_schema.model_extra or {}):
        descriptor = table_schema.model_dump(by_alias=True, exclude_unset=True)
        if get_is_frictionless_descriptor(descriptor, kind="tableSchema"):
            return assert_table_schema(descriptor)

    return table_schema
