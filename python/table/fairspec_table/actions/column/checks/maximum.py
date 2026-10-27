from __future__ import annotations

from dataclasses import dataclass
from typing import Callable

import polars as pl
from fairspec_metadata import CellExclusiveMaximumError, CellMaximumError
from fairspec_metadata import Column

from fairspec_table.helpers import stringify_number
from fairspec_table.models import CellMapping

from ._shared.value import get_numeric_value


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMaximumError | CellExclusiveMaximumError


def create_check_cell_maximum(
    *, is_exclusive: bool = False
) -> Callable[[Column, CellMapping], ColumnCheck | None]:
    def check_cell_maximum(column: Column, mapping: CellMapping) -> ColumnCheck | None:
        value_expr = get_numeric_value(column, mapping)
        if value_expr is None:
            return None
        if not hasattr(column.property, "maximum") and not hasattr(
            column.property, "exclusiveMaximum"
        ):
            return None

        maximum = (
            getattr(column.property, "exclusiveMaximum", None)
            if is_exclusive
            else getattr(column.property, "maximum", None)
        )
        if maximum is None:
            return None

        is_error_expr = value_expr.ge(maximum) if is_exclusive else value_expr.gt(maximum)

        error_template: CellMaximumError | CellExclusiveMaximumError
        if is_exclusive:
            error_template = CellExclusiveMaximumError(
                type="cell/exclusiveMaximum",
                columnName=column.name,
                maximum=stringify_number(maximum),
                rowNumber=0,
                cell="",
            )
        else:
            error_template = CellMaximumError(
                type="cell/maximum",
                columnName=column.name,
                maximum=stringify_number(maximum),
                rowNumber=0,
                cell="",
            )

        return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)

    return check_cell_maximum
