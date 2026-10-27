from __future__ import annotations

import polars as pl

from fairspec_metadata import EmailColumn

from fairspec_table.settings import EMAIL_REGEX


def parse_email_column(column: EmailColumn, column_expr: pl.Expr) -> pl.Expr:
    return (
        pl.when(column_expr.str.contains(EMAIL_REGEX))
        .then(column_expr)
        .otherwise(pl.lit(None))
        .alias(column.name)
    )
