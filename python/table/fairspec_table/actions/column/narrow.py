from __future__ import annotations

import polars as pl

from fairspec_metadata import CategoricalColumn, get_base_property_type

from fairspec_table.helpers import get_categorical_values_and_labels
from fairspec_table.models import ColumnMapping

INTEGER_VARIANTS = {pl.Int8, pl.Int16, pl.Int32, pl.Int64}
NUMBER_VARIANTS = {pl.Float32, pl.Float64}
STRING_VARIANTS = {pl.String, pl.Categorical, pl.Enum}
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
                    values,
                    labels,
                    default=None,
                    return_dtype=_get_categorical_dtype(mapping.target, values, labels),
                )

    return column_expr


# An Enum sorts, compares and takes MIN/MAX by category position: the listed order
# under withOrder, else value order, since labels sort as text ("10" < "2")
def _get_categorical_dtype(
    column: CategoricalColumn, values: list[str | int], labels: list[str]
) -> pl.DataType | type[pl.DataType]:
    if column.property.withOrder:
        return pl.Enum(list(dict.fromkeys(labels)))
    if get_base_property_type(column.property.type) == "integer":
        int_pairs = [
            (value, label)
            for value, label in zip(values, labels)
            if isinstance(value, int)
        ]
        return pl.Enum(
            list(dict.fromkeys(label for _, label in sorted(int_pairs, key=_get_key)))
        )
    if any(value != label for value, label in zip(values, labels)):
        str_pairs = [
            (value, label)
            for value, label in zip(values, labels)
            if isinstance(value, str)
        ]
        return pl.Enum(
            list(dict.fromkeys(label for _, label in sorted(str_pairs, key=_get_key)))
        )
    return pl.Categorical


def _get_key[T](pair: tuple[T, str]) -> T:
    return pair[0]
