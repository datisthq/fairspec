from __future__ import annotations

import polars as pl

from fairspec_table.models import ColumnMapping

from .narrow import narrow_column
from .parse import parse_column
from .substitute import substitute_column


def normalize_column(
    mapping: ColumnMapping,
    *,
    keep_type: bool = False,
) -> pl.Expr:
    column_expr = pl.col(mapping.source.name)
    column_expr = substitute_column(mapping, column_expr)

    if not keep_type:
        column_expr = parse_column(mapping, column_expr)
        column_expr = narrow_column(mapping, column_expr)

    return column_expr.alias(mapping.target.name)
