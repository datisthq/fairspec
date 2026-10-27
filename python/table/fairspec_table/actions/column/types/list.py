from __future__ import annotations

import polars as pl

from fairspec_metadata import ListColumn

from .integer import parse_integer_text
from .number import parse_number_text

TRUE_VALUES = ["true", "True", "TRUE", "1"]
FALSE_VALUES = ["false", "False", "FALSE", "0"]
DATE_PATTERN = r"^\d{4}-\d{2}-\d{2}$"


def parse_list_column(column: ListColumn, column_expr: pl.Expr) -> pl.Expr:
    delimiter = column.property.delimiter or ","
    item_type = column.property.itemType

    # An empty cell is an empty list, while every delimiter splits off an item, empty or not
    split_expr = column_expr.str.split(delimiter)
    items_expr = (
        pl.when(column_expr.eq(pl.lit("")))
        .then(split_expr.list.head(0))
        .otherwise(split_expr)
    )
    if not item_type or item_type == "string":
        return items_expr

    list_expr = items_expr.list.eval(_cast_item(item_type, pl.element()))

    # An unparsable item becomes null rather than failing, so a null item marks the cell
    return (
        pl.when(_count_items(list_expr, pl.element().is_null()) > 0)
        .then(pl.lit(None))
        .otherwise(list_expr)
    )


def stringify_list_column(column: ListColumn, column_expr: pl.Expr) -> pl.Expr:
    delimiter = column.property.delimiter or ","

    return column_expr.cast(pl.List(pl.String)).list.join(delimiter, ignore_nulls=True)


def _cast_item(item_type: str, item_expr: pl.Expr) -> pl.Expr:
    match item_type:
        case "integer":
            return parse_integer_text(item_expr)
        case "number":
            return parse_number_text(item_expr)
        case "boolean":
            return (
                pl.when(item_expr.is_in(TRUE_VALUES))
                .then(pl.lit(True))
                .when(item_expr.is_in(FALSE_VALUES))
                .then(pl.lit(False))
                .otherwise(pl.lit(None, dtype=pl.Boolean))
            )
        case "date-time":
            return item_expr.str.to_datetime(
                "%Y-%m-%dT%H:%M:%S%.f%#z", time_unit="us", time_zone="UTC", strict=False
            )
        case "local-date-time":
            return item_expr.str.to_datetime(
                "%Y-%m-%dT%H:%M:%S%.f", time_unit="us", strict=False
            )
        case "date":
            return item_expr.str.extract(DATE_PATTERN, 0).str.to_date(
                "%Y-%m-%d", strict=False
            )
        case "time":
            return (
                pl.concat_str([pl.lit("1970-01-01T"), item_expr])
                .str.to_datetime(
                    "%Y-%m-%dT%H:%M:%S%.f%#z",
                    time_unit="us",
                    time_zone="UTC",
                    strict=False,
                )
                .cast(pl.Time)
            )
        case "local-time":
            return item_expr.str.to_time("%H:%M:%S%.f", strict=False)
        case _:
            return item_expr


def _count_items(list_expr: pl.Expr, predicate_expr: pl.Expr) -> pl.Expr:
    return list_expr.list.eval(predicate_expr.cast(pl.UInt32)).list.sum()
