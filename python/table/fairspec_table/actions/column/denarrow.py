from __future__ import annotations

from typing import cast

import polars as pl

from fairspec_metadata import CategoricalColumn, get_base_property_type

from fairspec_table.helpers import get_categorical_values_and_labels
from fairspec_table.models import ColumnMapping

INTEGER_VARIANTS = {pl.Int8, pl.Int16, pl.Int32, pl.Int64}
NUMBER_VARIANTS = {pl.Float32, pl.Float64}
STRING_VARIANTS = {pl.String, pl.Categorical}
ALPHANUMERIC_VARIANTS = INTEGER_VARIANTS | NUMBER_VARIANTS | STRING_VARIANTS


def denarrow_column(mapping: ColumnMapping, column_expr: pl.Expr) -> pl.Expr:
    variant = mapping.source.type

    if isinstance(mapping.target, CategoricalColumn):
        if variant in ALPHANUMERIC_VARIANTS:
            target = mapping.target
            values, labels = get_categorical_values_and_labels(target)

            polars_type: type[pl.DataType] = (
                pl.String
                if get_base_property_type(cast(str, target.property.type)) == "string"
                else pl.Int64
            )

            if values:
                return column_expr.replace_strict(
                    labels, values, default=None, return_dtype=polars_type
                )

    return column_expr
