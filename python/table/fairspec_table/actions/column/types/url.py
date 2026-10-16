from __future__ import annotations

import polars as pl

from fairspec_metadata import UrlColumn

from fairspec_table.settings import URL_REGEX


def parse_url_column(column: UrlColumn, column_expr: pl.Expr) -> pl.Expr:
    return (
        pl.when(column_expr.str.contains(URL_REGEX))
        .then(column_expr)
        .otherwise(pl.lit(None))
        .alias(column.name)
    )
