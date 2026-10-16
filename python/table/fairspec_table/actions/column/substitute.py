from __future__ import annotations

import polars as pl

from fairspec_table.models import ColumnMapping, PolarsColumn


def substitute_column(mapping: ColumnMapping, column_expr: pl.Expr) -> pl.Expr:
    missing_value_type = _get_missing_value_type(mapping.source)
    if not missing_value_type:
        return column_expr

    flatten_missing_values = [
        item.value if hasattr(item, "value") else item
        for item in (mapping.target.property.missingValues or [])
    ]

    compatible_missing_values = (
        [
            value
            for value in flatten_missing_values
            if isinstance(value, str)
            if missing_value_type == "string"
        ]
        if missing_value_type == "string"
        else [
            value for value in flatten_missing_values if isinstance(value, (int, float))
        ]
    )

    if not compatible_missing_values:
        return column_expr

    return (
        pl.when(column_expr.is_in(compatible_missing_values))
        .then(pl.lit(None))
        .otherwise(column_expr)
        .alias(mapping.target.name)
    )


def _get_missing_value_type(polars_column: PolarsColumn) -> str | None:
    polars_type = polars_column.type

    if polars_type == pl.String:
        return "string"
    if polars_type in (pl.Int8, pl.Int16, pl.Int32, pl.Int64, pl.Float32, pl.Float64):
        return "number"

    return None
