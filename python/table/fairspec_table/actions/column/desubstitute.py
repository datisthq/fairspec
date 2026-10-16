from __future__ import annotations

from typing import Unpack

import polars as pl

from fairspec_metadata import get_base_property_type
from fairspec_metadata import Column

from fairspec_table.models import ColumnMapping, DenormalizeColumnOptions


def desubstitute_column(
    mapping: ColumnMapping,
    column_expr: pl.Expr,
    **options: Unpack[DenormalizeColumnOptions],
) -> pl.Expr:
    missing_value_type = _get_missing_value_type(mapping.target, **options)
    if not missing_value_type:
        return column_expr

    flatten_missing_values = [
        item.value if hasattr(item, "value") else item
        for item in (mapping.target.property.missingValues or [])
    ]

    compatible_missing_values = (
        [value for value in flatten_missing_values if isinstance(value, str)]
        if missing_value_type == "string"
        else [
            value for value in flatten_missing_values if isinstance(value, (int, float))
        ]
    )

    if not compatible_missing_values:
        return column_expr

    compatible_missing_value = compatible_missing_values[0]

    return (
        pl.when(column_expr.is_null())
        .then(pl.lit(compatible_missing_value))
        .otherwise(column_expr)
        .alias(mapping.target.name)
    )


def _get_missing_value_type(
    column: Column,
    **options: Unpack[DenormalizeColumnOptions],
) -> str | None:
    base_type = get_base_property_type(column.property.type)

    if base_type == "string":
        return "string"

    if base_type in ("integer", "number"):
        native_types = options.get("nativeTypes")
        return "number" if native_types and base_type in native_types else "string"

    return None
