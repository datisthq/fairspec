from __future__ import annotations

import re

import polars as pl

from fairspec_metadata import (
    IntegerCategoricalColumnProperty,
    IntegerColumn,
    IntegerColumnProperty,
)

from .._shared.text import strip_number_text

PLAIN_INTEGER_PATTERN = r"^-?(?:0|[1-9][0-9]*)$"
# NUMBER_PATTERN (number.py) with its sign, integer, fraction and exponent captured
NUMBER_PARTS_PATTERN = r"^(-?)(0|[1-9][0-9]*)(?:\.([0-9]+))?(?:[eE]([+-]?[0-9]+))?$"
MAX_INTEGER_DIGITS = 19
# Keeps the exponent arithmetic below from wrapping Int64
MAX_EXPONENT = 10**15


def parse_integer_column(
    column: IntegerColumn, column_expr: pl.Expr, *, is_plain: bool = False
) -> pl.Expr:
    text_expr = normalize_integer_text(column.property, column_expr)
    return (
        parse_plain_integer_text(text_expr) if is_plain else parse_integer_text(text_expr)
    )


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
    plain_expr = parse_plain_integer_text(text_expr)

    # Only a cell with a fraction or an exponent can be a whole number the plain form
    # misses, so the rest stay null there. Each step is a struct field, read by name: an
    # expression reused by value is copied into the tree, and every copy is evaluated
    parts_expr = (
        pl.when(text_expr.str.contains_any([".", "e", "E"]))
        .then(text_expr)
        .str.extract_groups(NUMBER_PARTS_PATTERN)
        .struct.rename_fields(["sign", "integer", "fraction", "exponent"])
    )
    exponent_expr = pl.field("exponent").cast(pl.Int64, strict=False)
    # The value is its significant digits (`core`) times 10 to the `shift`, whole when
    # the shift is not negative; Int128 holds every product of at most 19 digits
    other_value_expr = (
        parts_expr.struct.with_fields(
            digits=pl.concat_str(
                [pl.field("integer"), pl.field("fraction").fill_null("")]
            ),
            fraction_length=pl.field("fraction")
            .str.len_chars()
            .fill_null(0)
            .cast(pl.Int64),
            exponent=pl.when(pl.field("exponent").is_null())
            .then(pl.lit(0, dtype=pl.Int64))
            .otherwise(exponent_expr.clip(-MAX_EXPONENT, MAX_EXPONENT)),
        )
        .struct.with_fields(
            core=pl.field("digits").str.strip_chars("0"),
            significant=pl.field("digits").str.strip_chars_start("0"),
        )
        .struct.with_fields(
            core_length=pl.field("core").str.len_chars().cast(pl.Int64),
            shift=pl.field("exponent")
            - pl.field("fraction_length")
            + pl.field("significant").str.len_chars().cast(pl.Int64)
            - pl.field("core").str.len_chars().cast(pl.Int64),
        )
        .struct.with_fields(
            magnitude=pl.when(pl.field("core_length").eq(0))
            .then(pl.lit(0, dtype=pl.Int128))
            .when(
                pl.field("shift").ge(0)
                & (pl.field("core_length") + pl.field("shift")).le(MAX_INTEGER_DIGITS)
            )
            .then(
                pl.field("core").cast(pl.Int128, strict=False)
                * pl.lit(10, dtype=pl.Int128).pow(
                    pl.field("shift").clip(0, MAX_INTEGER_DIGITS)
                )
            )
        )
        .struct.with_fields(
            value=pl.when(pl.field("sign").eq("-"))
            .then(pl.lit(0, dtype=pl.Int128) - pl.field("magnitude"))
            .otherwise(pl.field("magnitude"))
            .cast(pl.Int64, strict=False)
        )
        .struct.field("value")
    )

    return pl.coalesce(plain_expr, other_value_expr)


def parse_plain_integer_text(text_expr: pl.Expr) -> pl.Expr:
    return (
        pl.when(text_expr.str.contains(PLAIN_INTEGER_PATTERN))
        .then(text_expr)
        .cast(pl.Int64, strict=False)
    )
