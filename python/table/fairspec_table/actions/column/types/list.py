from __future__ import annotations

import polars as pl

from fairspec_metadata import ListColumn

TRUE_VALUES = ["true", "True", "TRUE", "1"]
FALSE_VALUES = ["false", "False", "FALSE", "0"]


def parse_list_column(column: ListColumn, column_expr: pl.Expr) -> pl.Expr:
    delimiter = column.property.delimiter or ","
    item_type = column.property.itemType

    items_expr = column_expr.str.split(delimiter)
    if not item_type or item_type == "string":
        return items_expr

    list_expr = items_expr.list.eval(_parse_item(item_type, pl.element()))

    # An unparsable item becomes null rather than failing, and an empty one is null
    # by design, so a list holding more nulls than empty items had an invalid item
    return (
        pl.when(
            _count_items(list_expr, pl.element().is_null())
            > _count_items(items_expr, pl.element().eq(pl.lit("")))
        )
        .then(pl.lit(None))
        .otherwise(list_expr)
    )


def stringify_list_column(column: ListColumn, column_expr: pl.Expr) -> pl.Expr:
    delimiter = column.property.delimiter or ","

    return column_expr.cast(pl.List(pl.String)).list.join(delimiter, ignore_nulls=True)


def _parse_item(item_type: str, item_expr: pl.Expr) -> pl.Expr:
    return (
        pl.when(item_expr.eq(pl.lit("")))
        .then(pl.lit(None))
        .otherwise(_cast_item(item_type, item_expr))
    )


def _cast_item(item_type: str, item_expr: pl.Expr) -> pl.Expr:
    match item_type:
        case "integer":
            return item_expr.cast(pl.Int64, strict=False)
        case "number":
            return item_expr.cast(pl.Float64, strict=False)
        case "boolean":
            return (
                pl.when(item_expr.is_in(TRUE_VALUES))
                .then(pl.lit(True))
                .when(item_expr.is_in(FALSE_VALUES))
                .then(pl.lit(False))
                .otherwise(pl.lit(None, dtype=pl.Boolean))
            )
        case "date-time":
            return item_expr.str.to_datetime(strict=False)
        case "date":
            return item_expr.str.to_date("%Y-%m-%d", strict=False)
        case "time":
            return item_expr.str.to_time("%H:%M:%S", strict=False)
        case _:
            return item_expr


def _count_items(list_expr: pl.Expr, predicate_expr: pl.Expr) -> pl.Expr:
    return list_expr.list.eval(predicate_expr.cast(pl.UInt32)).list.sum()
