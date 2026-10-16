from __future__ import annotations

from typing import Unpack

import polars as pl
from fairspec_metadata import get_columns
from fairspec_metadata import TableSchema

from fairspec_table.actions.column.denormalize import denormalize_column
from fairspec_table.helpers.schema import get_polars_schema
from fairspec_table.models import (
    ColumnMapping,
    DenormalizeColumnOptions,
    SchemaMapping,
    Table,
)

from ._shared.helpers import merge_missing_values

HEAD_ROWS = 100


def denormalize_table(
    table: Table,
    table_schema: TableSchema,
    **options: Unpack[DenormalizeColumnOptions],
) -> Table:
    head: pl.DataFrame = table.head(HEAD_ROWS).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
    polars_schema = get_polars_schema(dict(head.schema))

    mapping = SchemaMapping(source=polars_schema, target=table_schema)
    return table.select(*denormalize_columns(mapping, **options).values())


def denormalize_columns(
    mapping: SchemaMapping,
    **options: Unpack[DenormalizeColumnOptions],
) -> dict[str, pl.Expr]:
    exprs: dict[str, pl.Expr] = {}
    columns = get_columns(mapping.target.model_dump())

    for column in columns:
        expr = pl.lit(None).alias(column.name)

        polars_column = next(
            (pc for pc in mapping.source.columns if pc.name == column.name),
            None,
        )

        if polars_column:
            merged_column = merge_missing_values(column, mapping.target)
            column_mapping = ColumnMapping(source=polars_column, target=merged_column)
            expr = denormalize_column(column_mapping, **options)

        exprs[column.name] = expr

    return exprs
