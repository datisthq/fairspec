from __future__ import annotations

from typing import TYPE_CHECKING, Unpack, cast

import polars as pl

from fairspec_metadata import (
    Resource,
    TableSchema,
    get_data_first_path,
    get_supported_file_dialect,
    resolve_table_schema,
)
from fairspec_metadata.models.file_dialect.file_dialect import FileDialect

from fairspec_table.actions.table.normalize import normalize_table
from fairspec_table.plugins.sqlite.actions.database.connect import connect_database
from fairspec_table.plugins.sqlite.actions.table_schema.infer import (
    infer_table_schema_from_sqlite,
)

if TYPE_CHECKING:
    from fairspec_table.models.table import LoadTableOptions, Table


def load_sqlite_table(resource: Resource, **options: Unpack[LoadTableOptions]) -> Table:
    first_path = get_data_first_path(resource)
    if not first_path:
        raise Exception("Resource path is not defined")

    file_dialect = get_supported_file_dialect(resource, ["sqlite"])
    if not file_dialect:
        raise Exception("Resource data is not compatible")

    conn = connect_database(first_path)
    try:
        cursor = conn.cursor()
        tables = cursor.execute(
            "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
        ).fetchall()

        table_name = getattr(file_dialect, "tableName", None) or (
            tables[0]["name"] if tables else None
        )

        if not table_name:
            raise Exception("Table name is not defined")

        rows = cursor.execute(f'SELECT * FROM "{table_name}"').fetchall()
        records = [dict(row) for row in rows]
        table: Table = pl.DataFrame(records).lazy()

        if not options.get("denormalized"):
            table_schema = resolve_table_schema(resource.tableSchema)
            if not table_schema:
                descriptor = infer_table_schema_from_sqlite(
                    Resource(data=first_path, fileDialect=cast(FileDialect, file_dialect))
                )
                table_schema = TableSchema.model_validate(descriptor)
            table = normalize_table(table, table_schema)

        return table
    finally:
        conn.close()
