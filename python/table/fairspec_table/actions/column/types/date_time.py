from __future__ import annotations

import polars as pl

from fairspec_metadata import DateTimeColumn

from ._shared.temporal import parse_default_temporal, reject_leap_second

DEFAULT_FORMAT = "%Y-%m-%dT%H:%M:%S%.fZ"
DEFAULT_PARSE_FORMAT = "%Y-%m-%dT%H:%M:%S%.f%#z"
DEFAULT_PATTERN = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?(?:Z|[+-][0-9]{2}:[0-9]{2})$"
DATE_TIME_TYPE = pl.Datetime("us", "UTC")


def parse_date_time_column(column: DateTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat
    if fmt:
        value_expr = column_expr.str.strptime(DATE_TIME_TYPE, fmt, strict=False)
        return reject_leap_second(column_expr, value_expr, fmt)

    return parse_date_time_text(column_expr).alias(column.name)


def parse_date_time_text(text_expr: pl.Expr) -> pl.Expr:
    return parse_default_temporal(
        text_expr, DEFAULT_PATTERN, DEFAULT_PARSE_FORMAT, DATE_TIME_TYPE
    )


def stringify_date_time_column(column: DateTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
