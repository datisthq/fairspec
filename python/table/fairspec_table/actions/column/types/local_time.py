from __future__ import annotations

import polars as pl

from fairspec_metadata import LocalTimeColumn

from ._shared.temporal import parse_default_temporal, reject_leap_second

DEFAULT_FORMAT = "%H:%M:%S%.f"
DEFAULT_PATTERN = r"^[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?$"
DATE_PREFIX = "1970-01-01T"
LOCAL_TIME_TYPE = pl.Datetime("us")


def parse_local_time_column(column: LocalTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat
    if not fmt:
        return parse_local_time_text(column_expr).alias(column.name)

    value_expr = (
        pl.concat_str([pl.lit(DATE_PREFIX), column_expr])
        .str.strptime(LOCAL_TIME_TYPE, f"%Y-%m-%dT{fmt}", strict=False)
        .cast(pl.Time)
    )
    return reject_leap_second(column_expr, value_expr, fmt).alias(column.name)


def parse_local_time_text(text_expr: pl.Expr) -> pl.Expr:
    return parse_default_temporal(
        text_expr,
        DEFAULT_PATTERN,
        f"%Y-%m-%dT{DEFAULT_FORMAT}",
        LOCAL_TIME_TYPE,
        prefix=DATE_PREFIX,
    ).cast(pl.Time)


def stringify_local_time_column(column: LocalTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
