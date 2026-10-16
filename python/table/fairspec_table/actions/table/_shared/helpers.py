from __future__ import annotations

from typing import cast

from fairspec_metadata import get_base_property_type
from fairspec_metadata import Column
from fairspec_metadata import TableSchema


def merge_missing_values(column: Column, table_schema: TableSchema) -> Column:
    if not table_schema.missingValues:
        return column

    merged_column = column.model_copy(deep=True)
    if merged_column.property.missingValues is None:
        merged_column.property.missingValues = []

    property_type = cast("str | list[str] | None", merged_column.property.type)

    for item in table_schema.missingValues:
        if get_base_property_type(property_type) == "string":
            value = item.value if hasattr(item, "value") else item
            if not isinstance(value, str):
                continue

        merged_column.property.missingValues.append(item)  # type: ignore[arg-type]

    return merged_column
