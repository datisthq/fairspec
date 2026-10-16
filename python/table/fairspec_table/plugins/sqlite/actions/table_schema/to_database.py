from __future__ import annotations

from fairspec_metadata import TableSchema, get_columns

from fairspec_table.plugins.sqlite.actions.column.to_database import (
    convert_column_to_database,
)
from fairspec_table.plugins.sqlite.models.schema import SqliteSchema


def convert_table_schema_to_database(
    table_schema: TableSchema, table_name: str
) -> SqliteSchema:
    schema = SqliteSchema(name=table_name, columns=[], isView=False)

    columns = get_columns(table_schema.model_dump())
    for column in columns:
        is_nullable = (
            column.nullable if column.nullable is not None else not column.required
        )
        database_column = convert_column_to_database(column, is_nullable)
        schema.columns.append(database_column)

    if table_schema.primaryKey:
        schema.primaryKey = table_schema.primaryKey

    return schema
