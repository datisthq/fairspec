from __future__ import annotations

from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellMissingError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMissingError


def check_cell_missing(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    if column.nullable:
        return None

    is_error_expr = mapping.target.is_null()

    error_template = CellMissingError(
        type="cell/missing",
        columnName=column.name,
        rowNumber=0,
        cell="",
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
