from __future__ import annotations

import polars as pl

from fairspec_metadata import Base64Column

from fairspec_table.settings import BASE64_REGEX


def parse_base64_column(column: Base64Column, column_expr: pl.Expr) -> pl.Expr:
    return (
        pl.when(column_expr.str.contains(BASE64_REGEX))
        .then(column_expr)
        .otherwise(pl.lit(None))
        .alias(column.name)
    )
