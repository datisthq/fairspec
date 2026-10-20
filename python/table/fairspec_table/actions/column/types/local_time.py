from __future__ import annotations

import polars as pl

from fairspec_metadata import LocalTimeColumn

DEFAULT_FORMAT = "%H:%M:%S%.f"
DEFAULT_PATTERN = r"^\d{2}:\d{2}:\d{2}(\.\d+)?$"


def parse_local_time_column(column: LocalTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat
    text_expr = column_expr if fmt else column_expr.str.extract(DEFAULT_PATTERN, 0)
    return (
        pl.concat_str([pl.lit("1970-01-01T"), text_expr])
        .str.strptime(
            pl.Datetime("us"), f"%Y-%m-%dT{fmt or DEFAULT_FORMAT}", strict=False
        )
        .cast(pl.Time)
        .alias(column.name)
    )


def stringify_local_time_column(column: LocalTimeColumn, column_expr: pl.Expr) -> pl.Expr:
    fmt = column.property.temporalFormat or DEFAULT_FORMAT
    return column_expr.dt.strftime(fmt)
