from __future__ import annotations

import polars as pl


def evaluate_expression(expr: pl.Expr) -> object:
    return pl.select(expr.alias("value")).to_dicts()[0]["value"]
