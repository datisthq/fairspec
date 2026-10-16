from __future__ import annotations

import polars as pl

from fairspec_metadata import BooleanColumn

DEFAULT_TRUE_VALUES = ["true", "True", "TRUE", "1"]
DEFAULT_FALSE_VALUES = ["false", "False", "FALSE", "0"]

DEFAULT_TRUE_VALUE = "true"
DEFAULT_FALSE_VALUE = "false"


def parse_boolean_column(column: BooleanColumn, column_expr: pl.Expr) -> pl.Expr:
    true_values = column.property.trueValues or DEFAULT_TRUE_VALUES
    false_values = column.property.falseValues or DEFAULT_FALSE_VALUES

    for value in true_values:
        column_expr = column_expr.str.replace(f"^{value}$", "1", literal=False)
    for value in false_values:
        column_expr = column_expr.str.replace(f"^{value}$", "0", literal=False)

    column_expr = column_expr.cast(pl.Int8, strict=False)

    return (
        pl.when(column_expr.eq(1))
        .then(pl.lit(True))
        .when(column_expr.eq(0))
        .then(pl.lit(False))
        .otherwise(pl.lit(None))
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
