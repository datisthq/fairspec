from __future__ import annotations

import polars as pl

from fairspec_metadata import LocalDateTimeColumn

from ._shared.temporal import parse_default_temporal, reject_leap_second

DEFAULT_FORMAT = "%Y-%m-%dT%H:%M:%S%.f"
DEFAULT_PATTERN = r"^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?$"
LOCAL_DATE_TIME_TYPE = pl.Datetime("us")


def parse_local_date_time_column(
    column: LocalDateTimeColumn, column_expr: pl.Expr
) -> pl.Expr:
    fmt = column.property.temporalFormat
    if fmt:
        value_expr = column_expr.str.strptime(LOCAL_DATE_TIME_TYPE, fmt, strict=False)
        return reject_leap_second(column_expr, value_expr, fmt)

    return parse_local_date_time_text(column_expr).alias(column.name)


def parse_local_date_time_text(text_expr: pl.Expr) -> pl.Expr:
    return parse_default_temporal(
        text_expr, DEFAULT_PATTERN, DEFAULT_FORMAT, LOCAL_DATE_TIME_TYPE
    )


def stringify_local_date_time_column(
    column: LocalDateTimeColumn, column_expr: pl.Expr
) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
