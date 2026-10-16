from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import create_column_from_property

from fairspec_table.plugins.sqlite.models.column import SqliteColumn

if TYPE_CHECKING:
    from fairspec_metadata import Column


def convert_column_from_database(database_column: SqliteColumn) -> Column:
    property = _convert_property(database_column.dataType)
    column = create_column_from_property(database_column.name, property)

    if database_column.comment:
        column.property.description = database_column.comment

    return column


def _convert_property(database_type: str) -> dict[str, str]:
    match database_type.lower():
        case "blob":
            return {"type": "string"}
        case "text":
            return {"type": "string"}
        case "integer":
            return {"type": "integer"}
        case "numeric" | "real":
            return {"type": "number"}
        case _:
            return {}
