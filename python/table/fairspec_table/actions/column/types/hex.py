from __future__ import annotations

import polars as pl

from fairspec_metadata import HexColumn

from fairspec_table.settings import HEX_REGEX


def parse_hex_column(column: HexColumn, column_expr: pl.Expr) -> pl.Expr:
    return (
        pl.when(column_expr.str.contains(HEX_REGEX))
        .then(column_expr)
        .otherwise(pl.lit(None))
        .alias(column.name)
    )
