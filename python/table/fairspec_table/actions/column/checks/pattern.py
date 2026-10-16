from __future__ import annotations

from dataclasses import dataclass

import polars as pl
from fairspec_metadata import CellPatternError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellPatternError


def check_cell_pattern(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    pattern = getattr(column.property, "pattern", None)
    if not pattern:
        return None

    is_error_expr = mapping.source.str.contains(pattern).not_()

    error_template = CellPatternError(
        type="cell/pattern",
        columnName=column.name,
        pattern=pattern,
        rowNumber=0,
        cell="",
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
