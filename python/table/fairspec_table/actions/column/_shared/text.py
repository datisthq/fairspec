from __future__ import annotations

import re

import polars as pl


def strip_number_text(
    column_expr: pl.Expr, *, decimal_char: str | None = None
) -> pl.Expr:
    """Strip the text around a number: the span from the first to the last digit is kept, and a
    minus in the stripped prefix makes it negative. Text inside the span stays, so it fails parsing
    """
    decimal = f"(?:{re.escape(decimal_char)})?" if decimal_char else ""
    span = f"({decimal}[0-9](?:.*[0-9])?)[^0-9]*$"
    negative = f"^[^0-9]*-[^0-9]*?{span}"
    positive = f"^[^0-9]*?{span}"

    return (
        pl.when(column_expr.str.contains(negative))
        .then(column_expr.str.replace(negative, "-$1"))
        .otherwise(column_expr.str.replace(positive, "$1"))
    )
