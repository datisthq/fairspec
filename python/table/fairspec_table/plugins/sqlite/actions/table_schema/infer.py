from __future__ import annotations

from fairspec_metadata import Resource, get_data_first_path, get_supported_file_dialect

from fairspec_table.plugins.sqlite.actions.database.connect import connect_database
from fairspec_table.plugins.sqlite.actions.table_schema.from_database import (
    convert_table_schema_from_database,
)
from fairspec_table.plugins.sqlite.models.column import SqliteColumn
from fairspec_table.plugins.sqlite.models.schema import SqliteSchema


def infer_table_schema_from_sqlite(resource: Resource) -> dict:
    first_path = get_data_first_path(resource)
    if not first_path:
        raise Exception("Database is not defined")

    dialect = get_supported_file_dialect(resource, ["sqlite"])
    if not dialect:
        raise Exception("Resource data is not compatible")

    conn = connect_database(first_path)
    try:
        cursor = conn.cursor()
        tables = cursor.execute(
            "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
        ).fetchall()

        table_name = getattr(dialect, "tableName", None) or (
            tables[0]["name"] if tables else None
        )

        if not table_name:
            raise Exception("Table name is not defined")

        pragma_rows = cursor.execute(f'PRAGMA table_info("{table_name}")').fetchall()

        columns: list[SqliteColumn] = []
        pk_columns: list[str] = []
        for row in pragma_rows:
            columns.append(
                SqliteColumn(
                    name=row["name"],
                    dataType=row["type"].lower() if row["type"] else "text",
                    isNullable=not bool(row["notnull"]),
                    hasDefaultValue=row["dflt_value"] is not None,
                )
            )
            if row["pk"]:
                pk_columns.append(row["name"])

        schema = SqliteSchema(
            name=table_name,
            columns=columns,
            primaryKey=pk_columns or None,
        )

        return convert_table_schema_from_database(schema)
    finally:
        conn.close()
