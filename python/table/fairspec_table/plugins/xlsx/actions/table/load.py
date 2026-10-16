from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

import polars as pl

from fairspec_dataset import load_file, prefetch_files
from fairspec_metadata import Resource, get_supported_file_dialect, resolve_table_schema

from fairspec_table.actions.data.file_dialect import get_records_from_rows
from fairspec_table.actions.table.normalize import normalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table
from fairspec_table.plugins.xlsx.actions.buffer.decode import decode_xlsx_buffer
from fairspec_table.plugins.xlsx.actions.file_dialect.infer import (
    infer_xlsx_file_dialect,
)

if TYPE_CHECKING:
    from fairspec_metadata.models.file_dialect.file_dialect import FileDialect

    from fairspec_table.models.table import LoadTableOptions, Table


def load_xlsx_table(resource: Resource, **options: Unpack[LoadTableOptions]) -> Table:
    file_dialect = get_supported_file_dialect(resource, ["xlsx", "ods"])
    if not file_dialect:
        raise Exception("Resource data is not compatible")

    paths = prefetch_files(resource, concurrency=options.get("concurrency"))
    if not paths:
        raise Exception("Resource path is not defined")

    if _dialect_has_only_format(file_dialect):
        inferred = infer_xlsx_file_dialect(resource.model_copy(update={"data": paths[0]}))
        if inferred:
            file_dialect = inferred

    format = getattr(file_dialect, "format", "xlsx")
    sheet_name = getattr(file_dialect, "sheetName", None)
    sheet_number = getattr(file_dialect, "sheetNumber", None)

    tables: list[Table] = []
    for path in paths:
        buffer = load_file(path)
        rows = decode_xlsx_buffer(
            buffer,
            format=format,
            sheet_name=sheet_name,
            sheet_number=sheet_number,
        )
        if rows:
            records = get_records_from_rows(rows, file_dialect)  # type: ignore[arg-type]
            table = pl.DataFrame(records).lazy()
            tables.append(table)

    result = pl.concat(tables)

    if not options.get("denormalized"):
        table_schema = resolve_table_schema(resource.tableSchema)
        if not table_schema:
            table_schema = infer_table_schema_from_table(result, **options)
        result = normalize_table(result, table_schema)

    return result


def _dialect_has_only_format(dialect: FileDialect) -> bool:
    keys = {
        k for k in type(dialect).model_fields if getattr(dialect, k, None) is not None
    }
    meaningful = keys - {"format", "type", "title", "description"}
    return len(meaningful) == 0
