from __future__ import annotations

from typing import Unpack

import polars as pl

from fairspec_table.models import ColumnMapping, DenormalizeColumnOptions

from .denarrow import denarrow_column
from .desubstitute import desubstitute_column
from .stringify import stringify_column


def denormalize_column(
    mapping: ColumnMapping,
    **options: Unpack[DenormalizeColumnOptions],
) -> pl.Expr:
    column_expr = pl.col(mapping.source.name)

    native_types = options.get("nativeTypes")
    if not native_types or mapping.target.type not in native_types:
        column_expr = denarrow_column(mapping, column_expr)
        column_expr = stringify_column(mapping, column_expr)

    column_expr = desubstitute_column(mapping, column_expr, **options)
    return column_expr
