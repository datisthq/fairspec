from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal

import polars as pl
from fairspec_metadata import CellMultipleOfError
from fairspec_metadata import Column, DecimalColumn, NumberColumn

from fairspec_table.models import CellMapping

from ..types.decimal import normalize_number_text

MAX_DECIMAL_DIGITS = 38
INTEGER_DECIMAL = pl.Decimal(MAX_DECIMAL_DIGITS, 0)
MAX_VALUE_DIGITS = MAX_DECIMAL_DIGITS - 1


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

    is_float_error_expr = (mapping.target.cast(pl.Float64) % multiple_of).eq(0).not_()

    integer_digits, _, fraction_digits = format(Decimal(str(multiple_of)), "f").partition(
        "."
    )
    scale = len(fraction_digits)
    divisor = f"{integer_digits}{fraction_digits}".lstrip("0")
    if not divisor or len(divisor) > MAX_DECIMAL_DIGITS or scale >= MAX_VALUE_DIGITS:
        return ColumnCheck(
            is_error_expr=is_float_error_expr, error_template=error_template
        )

    # The value scaled by 10^scale must be an integer of at most 37 digits: the quotient rounds,
    # so multiplied back it can pass 38 and abort the query; anything else is checked as a float
    text_expr = _select_value_text(column, mapping)
    is_exact_expr = text_expr.str.contains(
        f"^[+-]?0*[0-9]{{0,{MAX_VALUE_DIGITS - scale}}}(?:\\.[0-9]*)?$"
    ) & text_expr.str.contains("[0-9]")

    has_excess_digits_expr = text_expr.str.contains(f"\\.[0-9]{{{scale}}}[0-9]*[1-9]")
    integer_expr = text_expr.str.replace(r"\..*$", "").str.replace("^([+-]?)", "${1}0")
    fraction_expr = pl.concat_str(
        [text_expr.str.extract(r"\.([0-9]*)", 1).fill_null(""), pl.lit("0" * scale)]
    ).str.slice(0, scale)

    scaled_expr = (
        pl.when(is_exact_expr)
        .then(pl.concat_str([integer_expr, fraction_expr]))
        .otherwise(pl.lit("0"))
        .cast(INTEGER_DECIMAL)
    )

    divisor_expr = pl.lit(divisor).cast(INTEGER_DECIMAL)
    is_exact_error_expr = has_excess_digits_expr | (
        (scaled_expr / divisor_expr * divisor_expr).ne(scaled_expr)
    )

    is_error_expr = (
        pl.when(is_exact_expr).then(is_exact_error_expr).otherwise(is_float_error_expr)
    )

    return ColumnCheck(is_error_expr=is_error_expr, error_template=error_template)


# A float target keeps no digit past double precision, so the text as written is used
def _select_value_text(column: Column, mapping: CellMapping) -> pl.Expr:
    target_text_expr = mapping.target.cast(pl.String)
    if not isinstance(column, (NumberColumn, DecimalColumn)):
        return target_text_expr

    source_text_expr = normalize_number_text(column, mapping.source.cast(pl.String))
    return (
        pl.when(
            source_text_expr.cast(pl.Float64, strict=False).eq(
                mapping.target.cast(pl.Float64)
            )
        )
        .then(source_text_expr)
        .otherwise(target_text_expr)
    )
