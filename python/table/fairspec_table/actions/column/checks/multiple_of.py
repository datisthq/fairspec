from __future__ import annotations

from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellMultipleOfError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMultipleOfError


def check_cell_multiple_of(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    multiple_of = getattr(column.property, "multipleOf", None)
    if multiple_of is None:
        return None

    is_error_expr = (mapping.target % multiple_of).eq(0).not_()

    error_template = CellMultipleOfError(
        type="cell/multipleOf",
        columnName=column.name,
        multipleOf=multiple_of,
        rowNumber=0,
        cell="",
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
