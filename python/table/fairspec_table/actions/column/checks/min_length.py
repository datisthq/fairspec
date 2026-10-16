from __future__ import annotations

from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellMinLengthError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMinLengthError


def check_cell_min_length(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    min_length = getattr(column.property, "minLength", None)
    if not min_length:
        return None

    is_error_expr = mapping.source.str.len_chars().lt(min_length)

    error_template = CellMinLengthError(
        type="cell/minLength",
        columnName=column.name,
        minLength=min_length,
        rowNumber=0,
        cell="",
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
