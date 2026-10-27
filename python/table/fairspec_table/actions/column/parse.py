from __future__ import annotations

import polars as pl

from fairspec_metadata import (
    Base64Column,
    BooleanColumn,
    CategoricalColumn,
    DateColumn,
    DateTimeColumn,
    DecimalColumn,
    EmailColumn,
    HexColumn,
    IntegerCategoricalColumnProperty,
    IntegerColumn,
    ListColumn,
    LocalDateTimeColumn,
    LocalTimeColumn,
    NumberColumn,
    TimeColumn,
    UrlColumn,
)

from fairspec_table.models import ColumnMapping

from .types.base64 import parse_base64_column
from .types.boolean import parse_boolean_column
from .types.date import parse_date_column
from .types.date_time import parse_date_time_column
from .types.decimal import parse_decimal_column
from .types.email import parse_email_column
from .types.hex import parse_hex_column
from .types.integer import (
    normalize_integer_text,
    parse_integer_column,
    parse_integer_text,
)
from .types.list import parse_list_column
from .types.local_date_time import parse_local_date_time_column
from .types.local_time import parse_local_time_column
from .types.number import parse_number_column
from .types.time import parse_time_column
from .types.url import parse_url_column


def parse_column(mapping: ColumnMapping, column_expr: pl.Expr) -> pl.Expr:
    if mapping.source.type != pl.String:
        return column_expr

    column = mapping.target
    if isinstance(column, Base64Column):
        return parse_base64_column(column, column_expr)
    elif isinstance(column, BooleanColumn):
        return parse_boolean_column(column, column_expr)
    elif isinstance(column, CategoricalColumn):
        if isinstance(column.property, IntegerCategoricalColumnProperty):
            return parse_integer_text(
                normalize_integer_text(column.property, column_expr)
            )
        return column_expr
    elif isinstance(column, DateColumn):
        return parse_date_column(column, column_expr)
    elif isinstance(column, DateTimeColumn):
        return parse_date_time_column(column, column_expr)
    elif isinstance(column, DecimalColumn):
        return parse_decimal_column(column, column_expr)
    elif isinstance(column, EmailColumn):
        return parse_email_column(column, column_expr)
    elif isinstance(column, HexColumn):
        return parse_hex_column(column, column_expr)
    elif isinstance(column, IntegerColumn):
        return parse_integer_column(column, column_expr)
    elif isinstance(column, ListColumn):
        return parse_list_column(column, column_expr)
    elif isinstance(column, LocalDateTimeColumn):
        return parse_local_date_time_column(column, column_expr)
    elif isinstance(column, LocalTimeColumn):
        return parse_local_time_column(column, column_expr)
    elif isinstance(column, NumberColumn):
        return parse_number_column(column, column_expr)
    elif isinstance(column, TimeColumn):
        return parse_time_column(column, column_expr)
    elif isinstance(column, UrlColumn):
        return parse_url_column(column, column_expr)
    else:
        return column_expr
