from __future__ import annotations

import polars as pl

from fairspec_metadata import NumberColumn

from .decimal import normalize_number_text

NUMBER_PATTERN = r"^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?$"


def parse_number_column(column: NumberColumn, column_expr: pl.Expr) -> pl.Expr:
    column_expr = normalize_number_text(column, column_expr)

    # withText keeps a decimal point before the first digit as part of the number
    if column.property.withText:
        column_expr = column_expr.str.replace(r"^(-?)\.", "${1}0.")

    return parse_number_text(column_expr)


def stringify_number_column(_column: NumberColumn, column_expr: pl.Expr) -> pl.Expr:
    column_expr = column_expr.cast(pl.String)
    return column_expr


def parse_number_text(text_expr: pl.Expr) -> pl.Expr:
    """Read a JSON number as a Float64, or null when the text is not one"""
    return (
        pl.when(text_expr.str.contains(NUMBER_PATTERN))
        .then(text_expr)
        .cast(pl.Float64, strict=False)
    )
