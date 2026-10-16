from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_table.plugins.sqlite.models.column import SqliteColumn

if TYPE_CHECKING:
    from fairspec_metadata import Column


def convert_column_to_database(column: Column, is_nullable: bool = True) -> SqliteColumn:
    return SqliteColumn(
        name=column.name,
        dataType=_convert_type(column.type),
        isNullable=is_nullable,
        comment=column.property.description,
        isAutoIncrementing=False,
        hasDefaultValue=False,
    )


def _convert_type(column_type: str) -> str:
    match column_type:
        case "boolean":
            return "integer"
        case "integer":
            return "integer"
        case "number":
            return "real"
        case "string":
            return "text"
        case _:
            return "text"
