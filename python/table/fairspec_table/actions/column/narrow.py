from __future__ import annotations

import polars as pl

from fairspec_metadata import CategoricalColumn

from fairspec_table.helpers import get_categorical_values_and_labels
from fairspec_table.models import ColumnMapping

INTEGER_VARIANTS = {pl.Int8, pl.Int16, pl.Int32, pl.Int64}
NUMBER_VARIANTS = {pl.Float32, pl.Float64}
STRING_VARIANTS = {pl.String, pl.Categorical}
ALPHANUMERIC_VARIANTS = INTEGER_VARIANTS | NUMBER_VARIANTS | STRING_VARIANTS


def narrow_column(mapping: ColumnMapping, column_expr: pl.Expr) -> pl.Expr:
    variant = mapping.source.type

    if mapping.target.type == "boolean":
        if variant in INTEGER_VARIANTS:
            column_expr = (
                pl.when(column_expr.eq(1))
                .then(pl.lit(True))
                .when(column_expr.eq(0))
                .then(pl.lit(False))
                .otherwise(pl.lit(None))
            )

    if mapping.target.type == "integer":
        if variant in NUMBER_VARIANTS:
            column_expr = (
                pl.when(column_expr.eq(column_expr.round(0)))
                .then(column_expr.cast(pl.Int64))
                .otherwise(pl.lit(None))
            )

    if isinstance(mapping.target, CategoricalColumn):
        if variant in ALPHANUMERIC_VARIANTS:
            values, labels = get_categorical_values_and_labels(mapping.target)

            if values:
                return column_expr.replace_strict(
                    values, labels, default=None, return_dtype=pl.Categorical
                )

    return column_expr
