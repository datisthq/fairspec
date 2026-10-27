from __future__ import annotations

import re

import polars as pl

TOKEN_PATTERNS = {
    # Polars reads at most 4 digits of an unsigned year, so `%Y%m%d%H%M%S` splits like it
    "%Y": r"(?:[+-]\d+|\d{1,4})",
    "%j": r"\d{1,3}",
    "%S": r"(\d{1,2})",
    "%.f": r"(?:\.\d+)?",
    "%z": r"[+-]\d{2}:?\d{2}",
    "%:z": r"[+-]\d{2}:?\d{2}",
    "%b": "[a-z]+",
    "%B": "[a-z]+",
    "%p": "[a-z]+",
    "%%": "%",
}


LEAP_SECOND_PATTERN = r"([0-9]{2}:[0-9]{2}:)60"


def parse_default_temporal(
    text_expr: pl.Expr,
    pattern: str,
    fmt: str,
    dtype: pl.DataType,
    prefix: str = "",
) -> pl.Expr:
    """Reads an RFC 3339 default form: `T` and `Z` in any case, and a second `60` only at
    23:59 in UTC, read as 23:59:59"""
    matched_expr = text_expr.str.to_uppercase().str.extract(pattern, 0)
    value_expr = pl.concat_str(
        [pl.lit(prefix), matched_expr.str.replace(LEAP_SECOND_PATTERN, "${1}59")]
    ).str.strptime(dtype, fmt, strict=False)
    is_leap_second_expr = matched_expr.str.contains(LEAP_SECOND_PATTERN)

    return pl.when(
        is_leap_second_expr.not_()
        | (value_expr.dt.hour().eq(23) & value_expr.dt.minute().eq(59))
    ).then(value_expr)


def reject_leap_second(text_expr: pl.Expr, value_expr: pl.Expr, fmt: str) -> pl.Expr:
    """Nulls a value whose `%S` reads `60`, which Polars parses as a leap second"""
    pattern = _create_seconds_pattern(fmt)
    if pattern is None:
        return value_expr

    seconds_expr = text_expr.str.extract(pattern, 1)
    return pl.when(seconds_expr.eq_missing(pl.lit("60")).not_()).then(value_expr)


def _create_seconds_pattern(fmt: str) -> str | None:
    tokens = re.findall(r"%[.:]?.|[^%]", fmt, flags=re.S)
    if "%S" not in tokens:
        return None

    parts = []
    for token in tokens:
        if token in TOKEN_PATTERNS:
            parts.append(TOKEN_PATTERNS[token])
        elif token.startswith("%"):
            parts.append(r"\d{1,2}")
        else:
            parts.append(re.escape(token))

    return f"(?i)^{''.join(parts)}$"
