from __future__ import annotations

import polars as pl

from fairspec_metadata import NumberColumn

from .decimal import normalize_number_text


def parse_number_column(column: NumberColumn, column_expr: pl.Expr) -> pl.Expr:
    column_expr = normalize_number_text(column, column_expr)
    column_expr = column_expr.cast(pl.Float64, strict=False)
    return column_expr


def stringify_number_column(_column: NumberColumn, column_expr: pl.Expr) -> pl.Expr:
    column_expr = column_expr.cast(pl.String)
    return column_expr
