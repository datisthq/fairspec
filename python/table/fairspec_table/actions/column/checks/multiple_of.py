from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal

import polars as pl
from fairspec_metadata import CellMultipleOfError
from fairspec_metadata import Column

from fairspec_table.models import CellMapping

DECIMAL = pl.Decimal(38, 18)
MAX_DECIMAL_QUOTIENT = 1e18


@dataclass
class ColumnCheck:
    is_error_expr: pl.Expr
    error_template: CellMultipleOfError


def check_cell_multiple_of(column: Column, mapping: CellMapping) -> ColumnCheck | None:
    multiple_of = getattr(column.property, "multipleOf", None)
    if multiple_of is None:
        return None

    error_template = CellMultipleOfError(
        type="cell/multipleOf",
        columnName=column.name,
        multipleOf=multiple_of,
        rowNumber=0,
        cell="",
    )

    float_expr = mapping.target.cast(pl.Float64)
    is_float_error_expr = (float_expr % multiple_of).eq(0).not_()
    if multiple_of < 1 / MAX_DECIMAL_QUOTIENT or multiple_of >= MAX_DECIMAL_QUOTIENT:
        return ColumnCheck(
            is_error_expr=is_float_error_expr, error_template=error_template
        )

    # Binary floats make 99.99 % 0.01 non-zero, so the shortest text of the value is
    # divided as a decimal; a quotient past the decimal range would abort the query
    is_decimal_expr = (float_expr / multiple_of).abs().lt(MAX_DECIMAL_QUOTIENT)
    value_expr = (
        pl.when(is_decimal_expr)
        .then(mapping.target.cast(pl.String))
        .otherwise(pl.lit("0"))
        .cast(DECIMAL, strict=False)
    )

    divisor = format(Decimal(str(multiple_of)), "f")
    quotient_expr = value_expr / pl.lit(divisor).cast(DECIMAL)
    is_error_expr = (
        pl.when(is_decimal_expr)
        .then(quotient_expr.ne(quotient_expr.floor()))
        .otherwise(is_float_error_expr)
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)
