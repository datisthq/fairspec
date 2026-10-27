from __future__ import annotations

import polars as pl

from fairspec_metadata import DateColumn

from ._shared.temporal import reject_leap_second

DEFAULT_FORMAT = "%Y-%m-%d"
DEFAULT_PATTERN = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$"


def parse_date_column(column: DateColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat
    if fmt:
        value_expr = column_expr.str.strptime(pl.Date, fmt, strict=False)
        return reject_leap_second(column_expr, value_expr, fmt)

    return parse_date_text(column_expr)


def parse_date_text(text_expr: pl.Expr) -> pl.Expr:
    matched_expr = text_expr.str.extract(DEFAULT_PATTERN, 0)
    return matched_expr.str.strptime(pl.Date, DEFAULT_FORMAT, strict=False)


def stringify_date_column(column: DateColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
