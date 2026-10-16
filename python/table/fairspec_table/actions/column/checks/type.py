from __future__ import annotations

from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellTypeError
from fairspec_metadata import Column, ColumnType

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellTypeError


def check_cell_type(column: Column, mapping: CellMapping) -> ColumnCheck:
    is_error_expr = mapping.source.is_not_null() & mapping.target.is_null()

    error_template = CellTypeError(
        type="cell/type",
        columnName=column.name,
        columnType=ColumnType(column.type),
        rowNumber=0,
        cell="",
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
