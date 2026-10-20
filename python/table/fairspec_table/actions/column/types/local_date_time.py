from __future__ import annotations

import polars as pl

from fairspec_metadata import LocalDateTimeColumn

DEFAULT_FORMAT = "%Y-%m-%dT%H:%M:%S%.f"
DEFAULT_PATTERN = r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d+)?$"
LOCAL_DATE_TIME_TYPE = pl.Datetime("us")


def parse_local_date_time_column(
    column: LocalDateTimeColumn, column_expr: pl.Expr
) -> pl.Expr:
    fmt = column.property.temporalFormat
    if fmt:
        return column_expr.str.strptime(LOCAL_DATE_TIME_TYPE, fmt, strict=False)

    text_expr = column_expr.str.extract(DEFAULT_PATTERN, 0)
    return text_expr.str.strptime(LOCAL_DATE_TIME_TYPE, DEFAULT_FORMAT, strict=False)


def stringify_local_date_time_column(
    column: LocalDateTimeColumn, column_expr: pl.Expr
) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
