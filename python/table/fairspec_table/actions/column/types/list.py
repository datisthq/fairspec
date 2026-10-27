from __future__ import annotations

import polars as pl

from fairspec_metadata import ListColumn

from .date import parse_date_text
from .date_time import parse_date_time_text
from .integer import parse_integer_text
from .local_date_time import parse_local_date_time_text
from .local_time import parse_local_time_text
from .number import parse_number_text
from .time import parse_time_text

TRUE_VALUES = ["true", "True", "TRUE", "1"]
FALSE_VALUES = ["false", "False", "FALSE", "0"]


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
            return parse_date_time_text(item_expr)
        case "local-date-time":
            return parse_local_date_time_text(item_expr)
        case "date":
            return parse_date_text(item_expr)
        case "time":
            return parse_time_text(item_expr)
        case "local-time":
            return parse_local_time_text(item_expr)
        case _:
            return item_expr


def _count_items(list_expr: pl.Expr, predicate_expr: pl.Expr) -> pl.Expr:
    return list_expr.list.eval(predicate_expr.cast(pl.UInt32)).list.sum()
