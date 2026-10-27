from __future__ import annotations

from fairspec_metadata.actions.column.create import create_column_from_property
from fairspec_metadata.models.error.metadata import MetadataError
from fairspec_metadata.models.foreign_key import ForeignKey
from fairspec_metadata.models.table_schema import TableSchema

NUMERIC_COLUMN_TYPES = ("integer", "number", "decimal")


def inspect_foreign_key_types(
    foreign_key: ForeignKey,
    *,
    table_schema: TableSchema,
    referenced_schema: TableSchema,
    json_pointer: str,
) -> list[MetadataError]:
    """Check that each local column of a foreign key has the column type of the column it refers to"""
    for name, referenced_name in zip(foreign_key.columns, foreign_key.reference.columns):
        type = _get_column_type(table_schema, name)
        referenced_type = _get_column_type(referenced_schema, referenced_name)
        if type is None or referenced_type is None or type == referenced_type:
            continue
        if type in NUMERIC_COLUMN_TYPES and referenced_type in NUMERIC_COLUMN_TYPES:
            continue

        return [
            MetadataError(
                type="metadata",
                message=f'foreign key column "{name}" is {type}, but "{referenced_name}" is {referenced_type}',
                jsonPointer=json_pointer,
            )
        ]

    return []


def _get_column_type(table_schema: TableSchema, name: str) -> str | None:
    property = (table_schema.properties or {}).get(name)
    if property is None:
        return None
    descriptor = property.model_dump(mode="json", by_alias=True, exclude_none=True)
    return str(create_column_from_property(name, descriptor).type)
