from __future__ import annotations

import polars as pl

from fairspec_metadata import BooleanColumn

DEFAULT_TRUE_VALUES = ["true", "True", "TRUE", "1"]
DEFAULT_FALSE_VALUES = ["false", "False", "FALSE", "0"]

DEFAULT_TRUE_VALUE = "true"
DEFAULT_FALSE_VALUE = "false"


def parse_boolean_column(column: BooleanColumn, column_expr: pl.Expr) -> pl.Expr:
    true_values = column.property.trueValues
    false_values = column.property.falseValues
    if true_values is None:
        true_values = DEFAULT_TRUE_VALUES
    if false_values is None:
        false_values = DEFAULT_FALSE_VALUES

    return (
        pl.when(column_expr.is_in(true_values))
        .then(pl.lit(True))
        .when(column_expr.is_in(false_values))
        .then(pl.lit(False))
        .otherwise(pl.lit(None, dtype=pl.Boolean))
        .alias(column.name)
    )


def stringify_boolean_column(column: BooleanColumn, column_expr: pl.Expr) -> pl.Expr:
    true_value = (
        column.property.trueValues[0]
        if column.property.trueValues
        else DEFAULT_TRUE_VALUE
    )
    false_value = (
        column.property.falseValues[0]
        if column.property.falseValues
        else DEFAULT_FALSE_VALUE
    )

    return (
        pl.when(column_expr.eq(pl.lit(True)))
        .then(pl.lit(true_value))
        .otherwise(pl.lit(false_value))
        .alias(column.name)
    )
