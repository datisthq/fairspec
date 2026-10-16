from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import Resource, resolve_table_schema
from fairspec_table import denormalize_table
from fairspec_table.models.table import LoadTableOptions

from fairspec_library.actions.file_dialect.infer import infer_file_dialect
from fairspec_library.actions.table.load import load_table
from fairspec_library.actions.table_schema.infer import infer_table_schema

if TYPE_CHECKING:
    from fairspec_table import Table


def infer_table(resource: Resource, **options: Unpack[LoadTableOptions]) -> Table | None:
    resource = resource.model_copy(deep=True)

    if not resource.fileDialect:
        resource.fileDialect = infer_file_dialect(resource)

    table = load_table(resource, denormalized=True, **options)
    if table is None:
        return None

    table_schema = resolve_table_schema(resource.tableSchema)
    if not table_schema:
        table_schema = infer_table_schema(resource, **options)

    if table_schema:
        table = denormalize_table(table, table_schema)

    return table
