from __future__ import annotations

import polars as pl

from fairspec_metadata import DateTimeColumn

DEFAULT_FORMAT = "%Y-%m-%dT%H:%M:%S"
DEFAULT_PATTERN = r"^(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2})(\.\d+)?(Z|[+-]\d{2}:\d{2})?$"


# TODO: Add support for timezone handling
def parse_date_time_column(column: DateTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat
    if fmt:
        return column_expr.str.strptime(pl.Datetime, fmt, strict=False)

    text_expr = column_expr.str.extract(DEFAULT_PATTERN, 1)
    fraction_expr = column_expr.str.extract(DEFAULT_PATTERN, 2).fill_null("")
    return pl.concat_str([text_expr, fraction_expr]).str.strptime(
        pl.Datetime, f"{DEFAULT_FORMAT}%.f", strict=False
    )


def stringify_date_time_column(column: DateTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
