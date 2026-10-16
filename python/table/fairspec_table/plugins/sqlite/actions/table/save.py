from __future__ import annotations

import sqlite3
from typing import TYPE_CHECKING, Unpack, cast

import polars as pl

from fairspec_metadata import Resource, TableSchema, get_supported_file_dialect

from fairspec_table.actions.table.denormalize import denormalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table
from fairspec_table.plugins.sqlite.actions.database.connect import connect_database
from fairspec_table.plugins.sqlite.actions.table_schema.to_database import (
    convert_table_schema_to_database,
)
from fairspec_table.plugins.sqlite.models.schema import SqliteSchema
from fairspec_table.plugins.sqlite.settings import NATIVE_TYPES
from fairspec_table.settings import QUERY_ENGINE

if TYPE_CHECKING:
    from fairspec_table.models.table import SaveTableOptions, Table

BUFFER_SIZE = 10_000


def save_sqlite_table(table: Table, **options: Unpack[SaveTableOptions]) -> str:
    path = options["path"]

    resource = Resource(data=path, fileDialect=options.get("fileDialect"))
    file_dialect = get_supported_file_dialect(resource, ["sqlite"])
    if not file_dialect:
        raise Exception("Saving options is not compatible")

    table_schema = options.get("tableSchema")
    if not isinstance(table_schema, TableSchema):
        table_schema = infer_table_schema_from_table(table, **options, keepStrings=True)

    table = denormalize_table(table, table_schema, nativeTypes=NATIVE_TYPES)

    conn = connect_database(path, create=True)
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

        sqlite_schema = convert_table_schema_to_database(table_schema, table_name)

        _define_table(conn, sqlite_schema, overwrite=bool(options.get("overwrite")))
        _populate_table(conn, table_name, table)

        return path
    finally:
        conn.close()


def _define_table(
    conn: sqlite3.Connection, schema: SqliteSchema, *, overwrite: bool
) -> None:
    if overwrite:
        conn.execute(f'DROP TABLE IF EXISTS "{schema.name}"')

    cols: list[str] = []
    for col in schema.columns:
        null = "" if col.isNullable else " NOT NULL"
        cols.append(f'"{col.name}" {col.dataType}{null}')

    if schema.primaryKey:
        pk_cols = ", ".join(f'"{c}"' for c in schema.primaryKey)
        cols.append(f"PRIMARY KEY ({pk_cols})")

    sql = f'CREATE TABLE "{schema.name}" ({", ".join(cols)})'
    conn.execute(sql)


def _populate_table(conn: sqlite3.Connection, table_name: str, table: Table) -> None:
    frame = cast("pl.DataFrame", table.collect(engine=QUERY_ENGINE))
    records = frame.to_dicts()
    if not records:
        return

    columns = list(records[0].keys())
    placeholders = ", ".join("?" * len(columns))
    col_names = ", ".join(f'"{c}"' for c in columns)
    sql = f'INSERT INTO "{table_name}" ({col_names}) VALUES ({placeholders})'

    for i in range(0, len(records), BUFFER_SIZE):
        batch = records[i : i + BUFFER_SIZE]
        conn.executemany(sql, [tuple(r[c] for c in columns) for r in batch])

    conn.commit()
