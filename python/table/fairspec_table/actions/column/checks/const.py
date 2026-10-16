from __future__ import annotations

import json
from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellConstError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellConstError


def check_cell_const(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    const_constraint = column.property.const
    if const_constraint is None:
        return None

    primitive_const_constraint = (
        json.dumps(const_constraint)
        if isinstance(const_constraint, (dict, list))
        else const_constraint
    )

    is_error_expr = mapping.target.eq(pl.lit(primitive_const_constraint)).not_()

    error_template = CellConstError(
        type="cell/const",
        columnName=column.name,
        rowNumber=0,
        cell="",
        **{"const": str(primitive_const_constraint)},
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
