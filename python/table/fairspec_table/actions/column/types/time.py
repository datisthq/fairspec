from __future__ import annotations

import polars as pl

from fairspec_metadata import TimeColumn

DEFAULT_FORMAT = "%H:%M:%S"


def parse_time_column(column: TimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return (
        pl.concat_str([pl.lit("1970-01-01T"), column_expr])
        .str.strptime(pl.Datetime, f"%Y-%m-%dT{fmt}", strict=False)
        .cast(pl.Time)
        .alias(column.name)
    )


def stringify_time_column(column: TimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
