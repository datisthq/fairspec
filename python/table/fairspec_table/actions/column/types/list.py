from __future__ import annotations

import polars as pl

from fairspec_metadata import ListColumn


def parse_list_column(column: ListColumn, column_expr: pl.Expr) -> pl.Expr:
    delimiter = column.property.delimiter or ","
    item_type = column.property.itemType

    dtype = _get_item_dtype(item_type)

    column_expr = column_expr.str.split(delimiter)

    if dtype != pl.String:
        column_expr = column_expr.cast(pl.List(dtype), strict=False)

    return column_expr


def stringify_list_column(column: ListColumn, column_expr: pl.Expr) -> pl.Expr:
    delimiter = column.property.delimiter or ","

    return column_expr.cast(pl.List(pl.String)).list.join(delimiter, ignore_nulls=True)


ITEM_TYPE_MAP: dict[str, type[pl.DataType]] = {
    "integer": pl.Int64,
    "number": pl.Float64,
    "boolean": pl.Boolean,
    "date-time": pl.Datetime,
    "date": pl.Date,
    "time": pl.Time,
}


def _get_item_dtype(item_type: str | None) -> type[pl.DataType]:
    if item_type and item_type in ITEM_TYPE_MAP:
        return ITEM_TYPE_MAP[item_type]
    return pl.String
