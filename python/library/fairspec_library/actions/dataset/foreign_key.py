from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

import polars as pl
from fairspec_metadata import (
    FairspecError,
    ForeignKey,
    ForeignKeyError,
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

    for resource in dataset.resources or []:
        table_schema = resolve_table_schema(resource.tableSchema)
        if not table_schema:
            continue

        for foreign_key in table_schema.foreignKeys or []:
            fk_errors = _validate_foreign_key(resource, foreign_key, dataset, **options)
            errors.extend(fk_errors)

    return create_report(errors)


def _validate_foreign_key(
    resource: Resource,
    foreign_key: ForeignKey,
    dataset: Dataset,
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

    rename_mapping = dict(zip(ref_columns, columns))
    ref_selected = ref_table.select(
        [pl.col(name).alias(rename_mapping[name]) for name in ref_columns]
    ).unique(maintain_order=True)

    violations: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        table.select(columns)
        .drop_nulls()
        .join(ref_selected, on=columns, how="anti")
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
