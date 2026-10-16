from __future__ import annotations

from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellMaxLengthError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMaxLengthError


def check_cell_max_length(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    max_length = getattr(column.property, "maxLength", None)
    if not max_length:
        return None

    is_error_expr = mapping.source.str.len_chars().gt(max_length)

    error_template = CellMaxLengthError(
        type="cell/maxLength",
        columnName=column.name,
        maxLength=max_length,
        rowNumber=0,
        cell="",
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
