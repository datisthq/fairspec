from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

import polars as pl
from fairspec_metadata import get_data_records, resolve_table_schema
from fairspec_metadata import Resource

from fairspec_table.actions.table.normalize import normalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table

if TYPE_CHECKING:
    from fairspec_table.models.table import LoadTableOptions, Table


def load_inline_table(resource: Resource, **options: Unpack[LoadTableOptions]) -> Table:
    data_records = get_data_records(resource)
    if not data_records:
        raise Exception("Resource data is not defined or tabular")

    table = pl.DataFrame(data_records).lazy()

    if not options.get("denormalized"):
        table_schema = resolve_table_schema(resource.tableSchema)
        if not table_schema:
            table_schema = infer_table_schema_from_table(table, **options)
        table = normalize_table(table, table_schema)

    return table
