from __future__ import annotations

import polars as pl

from fairspec_metadata import DateColumn

DEFAULT_FORMAT = "%Y-%m-%d"


def parse_date_column(column: DateColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.str.strptime(pl.Date, fmt, strict=False)


def stringify_date_column(column: DateColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
