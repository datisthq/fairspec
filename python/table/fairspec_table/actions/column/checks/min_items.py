from __future__ import annotations

from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellMinItemsError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMinItemsError


def check_cell_min_items(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    if column.type != "list":
        return None

    min_items = getattr(column.property, "minItems", None)
    if min_items is None:
        return None

    is_error_expr = mapping.target.list.len().lt(min_items)

    error_template = CellMinItemsError(
        type="cell/minItems",
        columnName=column.name,
        minItems=min_items,
        rowNumber=0,
        cell="",
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
