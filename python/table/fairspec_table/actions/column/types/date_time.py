from __future__ import annotations

import polars as pl

from fairspec_metadata import DateTimeColumn

from ._shared.temporal import reject_leap_second

DEFAULT_FORMAT = "%Y-%m-%dT%H:%M:%S%.fZ"
DEFAULT_PARSE_FORMAT = "%Y-%m-%dT%H:%M:%S%.f%#z"
DEFAULT_PATTERN = r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?(Z|[+-]\d{2}:\d{2})$"
DATE_TIME_TYPE = pl.Datetime("us", "UTC")


def parse_date_time_column(column: DateTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat
    if fmt:
        value_expr = column_expr.str.strptime(DATE_TIME_TYPE, fmt, strict=False)
        return reject_leap_second(column_expr, value_expr, fmt)

    # %#z also reads `+02` and `+0200`, which the default form does not allow
    text_expr = column_expr.str.extract(DEFAULT_PATTERN, 0)
    return text_expr.str.strptime(DATE_TIME_TYPE, DEFAULT_PARSE_FORMAT, strict=False)


def stringify_date_time_column(column: DateTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
