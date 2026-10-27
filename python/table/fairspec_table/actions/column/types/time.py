from __future__ import annotations

from datetime import date

import polars as pl

from fairspec_metadata import TimeColumn

from ._shared.temporal import reject_leap_second

DEFAULT_FORMAT = "%H:%M:%S%.fZ"
DEFAULT_PARSE_FORMAT = "%H:%M:%S%.f%#z"
DEFAULT_PATTERN = r"^\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})$"


def parse_time_column(column: TimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat
    text_expr = column_expr if fmt else column_expr.str.extract(DEFAULT_PATTERN, 0)
    value_expr = (
        pl.concat_str([pl.lit("1970-01-01T"), text_expr])
        .str.strptime(
            pl.Datetime("us", "UTC"),
            f"%Y-%m-%dT{fmt or DEFAULT_PARSE_FORMAT}",
            strict=False,
        )
        .cast(pl.Time)
    )
    if fmt:
        value_expr = reject_leap_second(column_expr, value_expr, fmt)

    return value_expr.alias(column.name)


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
