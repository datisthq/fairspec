from __future__ import annotations

from dataclasses import dataclass
from typing import Callable

import polars as pl
from fairspec_metadata import CellExclusiveMinimumError, CellMinimumError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMinimumError | CellExclusiveMinimumError


def create_check_cell_minimum(
    *, is_exclusive: bool = False
) -> Callable[[Column, CellMapping], ColumnCheck | None]:
    def check_cell_minimum(column: Column, mapping: CellMapping) -> ColumnCheck | None:
        if not hasattr(column.property, "minimum") and not hasattr(
            column.property, "exclusiveMinimum"
        ):
            return None

        minimum = (
            getattr(column.property, "exclusiveMinimum", None)
            if is_exclusive
            else getattr(column.property, "minimum", None)
        )
        if minimum is None:
            return None

        is_error_expr = (
            mapping.target.le(minimum) if is_exclusive else mapping.target.lt(minimum)
        )

        error_template: CellMinimumError | CellExclusiveMinimumError
        if is_exclusive:
            error_template = CellExclusiveMinimumError(
                type="cell/exclusiveMinimum",
                columnName=column.name,
                minimum=str(minimum),
                rowNumber=0,
                cell="",
            )
        else:
            error_template = CellMinimumError(
                type="cell/minimum",
                columnName=column.name,
                minimum=str(minimum),
                rowNumber=0,
                cell="",
            )

        return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)

    return check_cell_minimum
