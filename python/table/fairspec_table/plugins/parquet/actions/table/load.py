from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

import polars as pl
from fairspec_dataset import prefetch_files
from fairspec_metadata import resolve_table_schema

from fairspec_table.actions.table.normalize import normalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table

if TYPE_CHECKING:
    from fairspec_metadata import Resource

    from fairspec_table.models.table import LoadTableOptions, Table


def load_parquet_table(resource: Resource, **options: Unpack[LoadTableOptions]) -> Table:
    paths = prefetch_files(resource, concurrency=options.get("concurrency"))
    if not paths:
        raise Exception("Resource data is not defined")

    first_path, *rest_paths = paths
    table = pl.scan_parquet(first_path)
    if rest_paths:
        table = pl.concat([table, *(pl.scan_parquet(path) for path in rest_paths)])

    if not options.get("denormalized"):
        table_schema = resolve_table_schema(resource.tableSchema)
        if not table_schema:
            table_schema = infer_table_schema_from_table(table, **options)
        table = normalize_table(table, table_schema)

    return table
