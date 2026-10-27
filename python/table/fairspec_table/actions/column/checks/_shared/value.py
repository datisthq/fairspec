from __future__ import annotations

import polars as pl
from fairspec_metadata import CategoricalColumn, Column, IntegerCategoricalColumnProperty

from fairspec_table.models import CellMapping

from ...types.integer import normalize_integer_text, parse_integer_text

NUMERIC_COLUMN_TYPES = ("integer", "number", "decimal")


def get_numeric_value(column: Column, mapping: CellMapping) -> pl.Expr | None:
    """The cell value a numeric constraint compares, or None for a column it ignores"""
    if column.type in NUMERIC_COLUMN_TYPES:
        return mapping.target

    # The target of a categorical column is its label, so the value is read again
    if isinstance(column, CategoricalColumn) and isinstance(
        column.property, IntegerCategoricalColumnProperty
    ):
        text_expr = mapping.source.cast(pl.String)
        return parse_integer_text(normalize_integer_text(column.property, text_expr))

    return None
