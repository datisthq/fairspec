from __future__ import annotations

import re

import polars as pl

from fairspec_metadata import (
    IntegerCategoricalColumnProperty,
    IntegerColumn,
    IntegerColumnProperty,
)

from .._shared.text import strip_number_text
from .number import NUMBER_PATTERN

PLAIN_INTEGER_PATTERN = r"^-?(?:0|[1-9][0-9]*)$"
MAX_INTEGER_DIGITS = 19
# Keeps the exponent arithmetic below from wrapping Int64 into a huge zero padding
MAX_EXPONENT = 10**15


def parse_integer_column(column: IntegerColumn, column_expr: pl.Expr) -> pl.Expr:
    return parse_integer_text(normalize_integer_text(column.property, column_expr))


def stringify_integer_column(_column: IntegerColumn, column_expr: pl.Expr) -> pl.Expr:
    column_expr = column_expr.cast(pl.String)
    return column_expr


def normalize_integer_text(
    property: IntegerColumnProperty | IntegerCategoricalColumnProperty,
    column_expr: pl.Expr,
) -> pl.Expr:
    if property.withText:
        column_expr = strip_number_text(column_expr)

    if property.groupChar:
        column_expr = column_expr.str.replace_all(re.escape(property.groupChar), "")

    return column_expr


def parse_integer_text(text_expr: pl.Expr) -> pl.Expr:
    """Read a JSON number whose fraction is zero as an Int64, or null outside its range"""
    plain_expr = (
        pl.when(text_expr.str.contains(PLAIN_INTEGER_PATTERN))
        .then(text_expr)
        .cast(pl.Int64, strict=False)
    )

    # Only cells the plain form missed reach the exact reading, so the rest stay null there
    other_expr = pl.when(plain_expr.is_null()).then(text_expr)
    other_expr = pl.when(other_expr.str.contains(NUMBER_PATTERN)).then(other_expr)

    fraction_expr = other_expr.str.extract(r"\.([0-9]+)", 1).fill_null("")
    digits_expr = pl.concat_str(
        [other_expr.str.extract(r"^-?([0-9]+)", 1), fraction_expr]
    ).str.strip_chars_start("0")
    trimmed_expr = digits_expr.str.strip_chars_end("0")
    exponent_expr = (
        other_expr.str.extract(r"[eE]([+-]?[0-9]+)$", 1)
        .fill_null("0")
        .cast(pl.Int64, strict=False)
        .clip(-MAX_EXPONENT, MAX_EXPONENT)
    )
    shift_expr = (
        exponent_expr
        - fraction_expr.str.len_chars().cast(pl.Int64)
        + digits_expr.str.len_chars().cast(pl.Int64)
        - trimmed_expr.str.len_chars().cast(pl.Int64)
    )
    length_expr = trimmed_expr.str.len_chars().cast(pl.Int64) + shift_expr

    is_whole_expr = shift_expr.ge(0) & length_expr.le(MAX_INTEGER_DIGITS)

    # repeat_by refuses a negative count even on rows the condition drops
    zeros_expr = (
        pl.lit("0")
        .repeat_by(pl.when(is_whole_expr).then(shift_expr).otherwise(pl.lit(0)))
        .list.join("")
    )
    whole_expr = (
        pl.when(trimmed_expr.eq(""))
        .then(pl.lit("0"))
        .when(is_whole_expr)
        .then(pl.concat_str([trimmed_expr, zeros_expr]))
    )
    sign_expr = (
        pl.when(other_expr.str.starts_with("-")).then(pl.lit("-")).otherwise(pl.lit(""))
    )
    other_value_expr = pl.concat_str([sign_expr, whole_expr]).cast(pl.Int64, strict=False)

    return pl.coalesce(plain_expr, other_value_expr)
