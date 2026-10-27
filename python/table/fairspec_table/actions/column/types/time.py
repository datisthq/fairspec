from __future__ import annotations

from datetime import date

import polars as pl

from fairspec_metadata import TimeColumn

from ._shared.temporal import parse_default_temporal, reject_leap_second

DEFAULT_FORMAT = "%H:%M:%S%.fZ"
DEFAULT_PARSE_FORMAT = "%H:%M:%S%.f%#z"
DEFAULT_PATTERN = r"^[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]+)?(?:Z|[+-][0-9]{2}:[0-9]{2})$"
DATE_PREFIX = "1970-01-01T"
TIME_TYPE = pl.Datetime("us", "UTC")


def parse_time_column(column: TimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat
    if not fmt:
        return parse_time_text(column_expr).alias(column.name)

    value_expr = (
        pl.concat_str([pl.lit(DATE_PREFIX), column_expr])
        .str.strptime(TIME_TYPE, f"%Y-%m-%dT{fmt}", strict=False)
        .cast(pl.Time)
    )
    return reject_leap_second(column_expr, value_expr, fmt).alias(column.name)


def parse_time_text(text_expr: pl.Expr) -> pl.Expr:
    return parse_default_temporal(
        text_expr,
        DEFAULT_PATTERN,
        f"%Y-%m-%dT{DEFAULT_PARSE_FORMAT}",
        TIME_TYPE,
        prefix=DATE_PREFIX,
    ).cast(pl.Time)


def stringify_time_column(column: TimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    # A Time has no zone, and an offset specification only formats a zoned date-time
    return (
        pl.lit(date(1970, 1, 1))
        .dt.combine(column_expr)
        .dt.replace_time_zone("UTC")
        .dt.strftime(fmt)
        .alias(column.name)
    )
