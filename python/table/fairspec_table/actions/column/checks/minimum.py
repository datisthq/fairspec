from __future__ import annotations

from dataclasses import dataclass
from typing import Callable

import polars as pl
from fairspec_metadata import CellExclusiveMinimumError, CellMinimumError
from fairspec_metadata import Column

from fairspec_table.helpers import stringify_number
from fairspec_table.models import CellMapping

from ._shared.value import get_numeric_value


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMinimumError | CellExclusiveMinimumError


def create_check_cell_minimum(
    *, is_exclusive: bool = False
) -> Callable[[Column, CellMapping], ColumnCheck | None]:
    def check_cell_minimum(column: Column, mapping: CellMapping) -> ColumnCheck | None:
        value_expr = get_numeric_value(column, mapping)
        if value_expr is None:
            return None
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

        is_error_expr = value_expr.le(minimum) if is_exclusive else value_expr.lt(minimum)

        error_template: CellMinimumError | CellExclusiveMinimumError
        if is_exclusive:
            error_template = CellExclusiveMinimumError(
                type="cell/exclusiveMinimum",
                columnName=column.name,
                minimum=stringify_number(minimum),
                rowNumber=0,
                cell="",
            )
        else:
            error_template = CellMinimumError(
                type="cell/minimum",
                columnName=column.name,
                minimum=stringify_number(minimum),
                rowNumber=0,
                cell="",
            )

        return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)

    return check_cell_minimum
