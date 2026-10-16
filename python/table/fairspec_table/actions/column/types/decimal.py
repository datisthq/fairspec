from __future__ import annotations

import re

import polars as pl

from fairspec_metadata import DecimalColumn


def parse_decimal_column(column: DecimalColumn, column_expr: pl.Expr) -> pl.Expr:
    decimal_char = column.property.decimalChar or "."
    group_char = column.property.groupChar or ""
    with_text = column.property.withText

    if group_char == "." and decimal_char == ",":
        column_expr = column_expr.str.replace_all(",", "###DECIMAL###")
        column_expr = column_expr.str.replace_all(r"\.", "")
        column_expr = column_expr.str.replace_all("###DECIMAL###", ".")
    else:
        if group_char:
            escaped_group_char = re.escape(group_char)
            column_expr = column_expr.str.replace_all(escaped_group_char, "")

        if decimal_char and decimal_char != ".":
            column_expr = column_expr.str.replace_all(re.escape(decimal_char), ".")

    if with_text:
        column_expr = column_expr.str.replace_all(r"[^\d\-.e]", "")

    column_expr = column_expr.cast(pl.Decimal(scale=18), strict=False)
    return column_expr


def stringify_decimal_column(_column: DecimalColumn, column_expr: pl.Expr) -> pl.Expr:
    column_expr = column_expr.cast(pl.String)
    return column_expr
