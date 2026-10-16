from __future__ import annotations

import json
from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellEnumError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellEnumError


def check_cell_enum(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    enum_constraint = column.property.enum
    if enum_constraint is None:
        return None

    primitive_enum_constraint = [
        json.dumps(item) if isinstance(item, (dict, list)) else item
        for item in enum_constraint
    ]

    is_error_expr = mapping.target.is_in(primitive_enum_constraint).not_()

    error_template = CellEnumError(
        type="cell/enum",
        columnName=column.name,
        enum=[str(item) for item in enum_constraint],
        rowNumber=0,
        cell="",
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
