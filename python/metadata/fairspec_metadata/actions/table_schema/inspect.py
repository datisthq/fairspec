from __future__ import annotations

from fairspec_metadata.models.error.metadata import MetadataError
from fairspec_metadata.models.table_schema import TableSchema


def inspect_table_schema(
    table_schema: TableSchema, *, root_json_pointer: str | None = None
) -> list[MetadataError]:
    """Check the key definitions of a table schema against its columns"""
    root = "" if root_json_pointer in (None, "/") else root_json_pointer
    names = set((table_schema.properties or {}).keys())
    errors: list[MetadataError] = []

    def check_columns(columns: list[str], pointer: str) -> None:
        for index, column in enumerate(columns):
            if column not in names:
                errors.append(
                    MetadataError(
                        type="metadata",
                        message=f'column "{column}" is not in properties',
                        jsonPointer=f"{root}{pointer}/{index}",
                    )
                )

    check_columns(table_schema.primaryKey or [], "/primaryKey")
    for index, unique_key in enumerate(table_schema.uniqueKeys or []):
        check_columns(unique_key, f"/uniqueKeys/{index}")
    for index, foreign_key in enumerate(table_schema.foreignKeys or []):
        check_columns(foreign_key.columns, f"/foreignKeys/{index}/columns")
        if len(foreign_key.columns) != len(foreign_key.reference.columns):
            errors.append(
                MetadataError(
                    type="metadata",
                    message="foreign key columns and reference columns differ in length",
                    jsonPointer=f"{root}/foreignKeys/{index}/reference/columns",
                )
            )

    return errors
