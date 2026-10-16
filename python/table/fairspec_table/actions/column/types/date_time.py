from __future__ import annotations

import polars as pl

from fairspec_metadata import DateTimeColumn

DEFAULT_FORMAT = "%Y-%m-%dT%H:%M:%S"


def parse_date_time_column(column: DateTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.str.strptime(pl.Datetime, fmt, strict=False)


def stringify_date_time_column(column: DateTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
