from __future__ import annotations

import polars as pl

from fairspec_metadata import (
    BooleanColumn,
    DateColumn,
    DateTimeColumn,
    DecimalColumn,
    IntegerColumn,
    ListColumn,
    NumberColumn,
    TimeColumn,
    UnknownColumn,
)

from fairspec_table.models import ColumnMapping

from .types.boolean import stringify_boolean_column
from .types.date import stringify_date_column
from .types.date_time import stringify_date_time_column
from .types.decimal import stringify_decimal_column
from .types.integer import stringify_integer_column
from .types.list import stringify_list_column
from .types.number import stringify_number_column
from .types.time import stringify_time_column
from .types.unknown import stringify_unknown_column


def stringify_column(mapping: ColumnMapping, column_expr: pl.Expr) -> pl.Expr:
    if mapping.source.type == pl.String:
        return column_expr

    column = mapping.target
    if isinstance(column, BooleanColumn):
        return stringify_boolean_column(column, column_expr)
    elif isinstance(column, DateColumn):
        return stringify_date_column(column, column_expr)
    elif isinstance(column, DateTimeColumn):
        return stringify_date_time_column(column, column_expr)
    elif isinstance(column, DecimalColumn):
        return stringify_decimal_column(column, column_expr)
    elif isinstance(column, IntegerColumn):
        return stringify_integer_column(column, column_expr)
    elif isinstance(column, ListColumn):
        return stringify_list_column(column, column_expr)
    elif isinstance(column, NumberColumn):
        return stringify_number_column(column, column_expr)
    elif isinstance(column, TimeColumn):
        return stringify_time_column(column, column_expr)
    elif isinstance(column, UnknownColumn):
        return stringify_unknown_column(column, column_expr)
    else:
        return column_expr
