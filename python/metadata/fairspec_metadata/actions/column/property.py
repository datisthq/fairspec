from __future__ import annotations

from collections.abc import Sequence

from fairspec_metadata.models.column.column import Column, ColumnProperty


def get_base_property_type(type: str | Sequence[str] | None) -> str | None:
    if type is None:
        return None
    if isinstance(type, str):
        return type
    for t in type:
        if t != "null":
            return t
    return "null"


def get_is_nullable_property_type(type: str | Sequence[str] | None) -> bool:
    if type is None:
        return False
    if isinstance(type, str):
        return False
    return "null" in type


def get_column_properties(columns: list[Column]) -> dict[str, ColumnProperty]:
    return {column.name: column.property for column in columns}


def set_property_nullable(column: Column) -> None:
    """Widen a column property type to include "null"."""
    base_type = column.property.type
    if base_type and isinstance(base_type, str):
        column.property.type = (base_type, "null")  # type: ignore[assignment]
