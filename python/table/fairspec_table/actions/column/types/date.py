from __future__ import annotations

import polars as pl

from fairspec_metadata import DateColumn

from ._shared.temporal import reject_leap_second

DEFAULT_FORMAT = "%Y-%m-%d"
DEFAULT_PATTERN = r"^\d{4}-\d{2}-\d{2}$"


def parse_date_column(column: DateColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat
    if fmt:
        value_expr = column_expr.str.strptime(pl.Date, fmt, strict=False)
        return reject_leap_second(column_expr, value_expr, fmt)

    text_expr = column_expr.str.extract(DEFAULT_PATTERN, 0)
    return text_expr.str.strptime(pl.Date, DEFAULT_FORMAT, strict=False)


def stringify_date_column(column: DateColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
