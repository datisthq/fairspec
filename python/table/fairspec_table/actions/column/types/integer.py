from __future__ import annotations

import re

import polars as pl

from fairspec_metadata import IntegerColumn


def parse_integer_column(column: IntegerColumn, column_expr: pl.Expr) -> pl.Expr:
    group_char = column.property.groupChar
    with_text = column.property.withText

    if with_text:
        column_expr = column_expr.str.replace_all(r"^[^\d\-]+", "")
        column_expr = column_expr.str.replace_all(r"[^\d\-]+$", "")

    if group_char:
        escaped_group_char = re.escape(group_char)
        column_expr = column_expr.str.replace_all(escaped_group_char, "")

    column_expr = column_expr.cast(pl.Int64, strict=False)
    return column_expr


def stringify_integer_column(_column: IntegerColumn, column_expr: pl.Expr) -> pl.Expr:
    column_expr = column_expr.cast(pl.String)
    return column_expr
