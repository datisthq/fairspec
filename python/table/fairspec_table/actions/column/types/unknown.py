from __future__ import annotations

import polars as pl

from fairspec_metadata import UnknownColumn


def stringify_unknown_column(_column: UnknownColumn, column_expr: pl.Expr) -> pl.Expr:
    return column_expr.cast(pl.String)
