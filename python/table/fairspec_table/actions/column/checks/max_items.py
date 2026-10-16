from __future__ import annotations

from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellMaxItemsError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMaxItemsError


def check_cell_max_items(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    if column.type != "list":
        return None

    max_items = getattr(column.property, "maxItems", None)
    if max_items is None:
        return None

    is_error_expr = mapping.target.list.len().gt(max_items)

    error_template = CellMaxItemsError(
        type="cell/maxItems",
        columnName=column.name,
        maxItems=max_items,
        rowNumber=0,
        cell="",
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
