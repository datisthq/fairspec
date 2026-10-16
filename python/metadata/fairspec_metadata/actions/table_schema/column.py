from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.actions.column.create import create_column_from_property

if TYPE_CHECKING:
    from fairspec_metadata.models.column.column import Column
    from fairspec_metadata.models.descriptor import Descriptor


def get_columns(table_schema: Descriptor) -> list[Column]:
    columns: list[Column] = []

    for name, property in (table_schema.get("properties") or {}).items():
        column = create_column_from_property(name, property)
        column.required = (
            table_schema.get("allRequired")
            or (name in (table_schema.get("required") or []))
            or None
        )
        columns.append(column)

    return columns
