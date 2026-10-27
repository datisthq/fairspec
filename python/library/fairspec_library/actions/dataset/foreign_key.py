from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

import polars as pl
from fairspec_metadata import (
    FairspecError,
    ForeignKey,
    ForeignKeyError,
    MetadataError,
    Report,
    ResourceMissingError,
    create_report,
    resolve_table_schema,
)

from fairspec_table.settings import QUERY_ENGINE

from fairspec_library.actions.table.load import load_table
from fairspec_library.models.table import ValidateTableOptions

if TYPE_CHECKING:
    from fairspec_metadata import Dataset, Resource


def validate_dataset_foreign_keys(
    dataset: Dataset, **options: Unpack[ValidateTableOptions]
) -> Report:
    errors: list[FairspecError] = []

    for index, resource in enumerate(dataset.resources or []):
        table_schema = resolve_table_schema(resource.tableSchema)
        if not table_schema:
            continue

        for fk_index, foreign_key in enumerate(table_schema.foreignKeys or []):
            fk_errors = _validate_foreign_key(
                resource,
                foreign_key,
                dataset,
                json_pointer=f"/resources/{index}/tableSchema/foreignKeys/{fk_index}",
                **options,
            )
            errors.extend(fk_errors)

    return create_report(errors)


def _validate_foreign_key(
    resource: Resource,
    foreign_key: ForeignKey,
    dataset: Dataset,
    *,
    json_pointer: str,
    **options: Unpack[ValidateTableOptions],
) -> list[FairspecError]:
    reference = foreign_key.reference
    columns = foreign_key.columns
    ref_columns = reference.columns

    if not columns or not ref_columns or len(columns) != len(ref_columns):
        return []

    ref_resource = (
        _find_resource(dataset, reference.resource) if reference.resource else resource
    )
    if not ref_resource:
        return [
            ResourceMissingError(
                type="resource/missing",
                resourceName=reference.resource,
                referencingResourceName=resource.name or "",
            )
        ]

    table = load_table(resource, **options)
    if table is None:
        return []

    ref_table = load_table(ref_resource, **options)
    if ref_table is None:
        return []

    key_types = _get_key_types(table, ref_table, columns, ref_columns)
    if key_types is None:
        return [
            MetadataError(
                type="metadata",
                message="foreign key columns and reference columns differ in type",
                jsonPointer=json_pointer,
            )
        ]

    key_names = [f"fairspec:key:{index}" for index in range(len(columns))]
    ref_selected = ref_table.select(
        [
            _cast_key(pl.col(name), key_type).alias(key_name)
            for name, key_type, key_name in zip(ref_columns, key_types, key_names)
        ]
    ).unique(maintain_order=True)

    violations: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        table.select(columns)
        .drop_nulls()
        .with_columns(
            _cast_key(pl.col(name), key_type).alias(key_name)
            for name, key_type, key_name in zip(columns, key_types, key_names)
        )
        .join(ref_selected, on=key_names, how="anti")
        .select(columns)
        .unique(maintain_order=True)
        .collect(engine=QUERY_ENGINE)
    )

    errors: list[FairspecError] = []
    for row in violations.to_dicts():
        cells = [str(row[c]) for c in columns]
        errors.append(
            ForeignKeyError(
                type="foreignKey",
                resourceName=resource.name,
                foreignKey=foreign_key,
                cells=cells,
            )
        )

    return errors


def _find_resource(dataset: Dataset, name: str) -> Resource | None:
    for resource in dataset.resources or []:
        if resource.name == name:
            return resource

    return None


def _get_key_types(
    table: pl.LazyFrame,
    ref_table: pl.LazyFrame,
    columns: list[str],
    ref_columns: list[str],
) -> list[pl.DataType | None] | None:
    """The type each key pair joins on: numeric columns of differing types compare by
    value, any other differing pair is None, which never joins"""
    schema = table.collect_schema()
    ref_schema = ref_table.collect_schema()
    key_types: list[pl.DataType | None] = []

    for name, ref_name in zip(columns, ref_columns):
        dtype = schema.get(name)
        ref_dtype = ref_schema.get(ref_name)
        if dtype is None or ref_dtype is None:
            return None
        if isinstance(dtype, (pl.Categorical, pl.Enum)):
            if not isinstance(ref_dtype, (pl.Categorical, pl.Enum)):
                return None
            key_types.append(pl.String())
        elif dtype.is_numeric() and ref_dtype.is_numeric():
            if dtype == ref_dtype:
                key_types.append(None)
            elif dtype.is_float() or ref_dtype.is_float():
                key_types.append(pl.Float64())
            else:
                key_types.append(pl.Decimal(38, 18))
        elif dtype == ref_dtype:
            key_types.append(None)
        else:
            return None

    return key_types


def _cast_key(column_expr: pl.Expr, key_type: pl.DataType | None) -> pl.Expr:
    return column_expr if key_type is None else column_expr.cast(key_type)
