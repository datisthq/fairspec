from __future__ import annotations

from typing import TYPE_CHECKING, Unpack, cast

import polars as pl

from fairspec_dataset import load_file, prefetch_files
from fairspec_metadata import (
    JsonFileDialect,
    JsonlFileDialect,
    Resource,
    get_supported_file_dialect,
    resolve_table_schema,
)
from fairspec_metadata.models.file_dialect.common import RowType

from fairspec_table.actions.data.file_dialect import get_records_from_rows
from fairspec_table.actions.table.normalize import normalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table
from fairspec_table.helpers.general import get_is_object
from fairspec_table.plugins.json.actions.buffer.decode import decode_json_buffer
from fairspec_table.plugins.json.actions.file_dialect.infer import (
    infer_json_file_dialect,
)
from fairspec_table.plugins.json.helpers.pointer import parse_json_pointer

if TYPE_CHECKING:
    from fairspec_metadata.models.file_dialect.file_dialect import FileDialect

    from fairspec_table.models.data import DataRecord, DataRow
    from fairspec_table.models.table import LoadTableOptions, Table


def load_json_table(
    resource: Resource, **options: Unpack[LoadTableOptions]
) -> Table | None:
    """Load a JSON or JSONL file as a table, or None when its rows are not an array"""
    file_dialect = get_supported_file_dialect(resource, ["json", "jsonl"])
    if not file_dialect:
        raise Exception("Resource data is not compatible")

    is_lines = getattr(file_dialect, "format", None) == "jsonl"
    max_bytes = options.get("previewBytes") if is_lines else None
    paths = prefetch_files(
        resource, max_bytes=max_bytes, concurrency=options.get("concurrency")
    )
    if not paths:
        raise Exception("Resource data is not defined")

    if _dialect_has_only_format(file_dialect):
        inferred = infer_json_file_dialect(
            Resource(data=paths[0], fileDialect=cast("FileDialect", file_dialect))
        )
        if inferred:
            file_dialect = inferred

    is_lines = getattr(file_dialect, "format", None) == "jsonl"
    is_default = _is_default_dialect(file_dialect)

    tables: list[Table] = []
    for path in paths:
        if is_lines and is_default:
            table = pl.scan_ndjson(path)
            tables.append(table)
            continue

        buffer = load_file(path)
        records = _get_records_from_json(
            decode_json_buffer(buffer, is_lines=is_lines), file_dialect
        )
        if records is None:
            return None
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


def _is_default_dialect(dialect: FileDialect) -> bool:
    for key in type(dialect).model_fields:
        if key in ("format", "type", "title", "description"):
            continue
        if getattr(dialect, key, None) is not None:
            return False
    return True


def _get_records_from_json(
    document: object,
    dialect: FileDialect,
) -> list[DataRecord] | None:
    if not isinstance(dialect, (JsonFileDialect, JsonlFileDialect)):
        return None

    data = document
    if isinstance(dialect, JsonFileDialect) and dialect.jsonPointer is not None:
        data = _get_json_pointer_value(document, dialect.jsonPointer)
    if not isinstance(data, list):
        return None

    row_type = dialect.rowType or (
        RowType.array if data and isinstance(data[0], list) else RowType.object
    )
    if row_type == RowType.array:
        rows: list[DataRow] = [list(row) for row in data if isinstance(row, list)]
        return get_records_from_rows(rows, dialect)

    records = [row for row in data if get_is_object(row)]
    column_names = dialect.columnNames
    if not column_names:
        return records

    return [{name: row.get(name) for name in column_names} for row in records]


def _get_json_pointer_value(document: object, pointer: str) -> object:
    value = document
    for token in parse_json_pointer(pointer):
        if isinstance(value, list):
            if not token.isdigit() or int(token) >= len(value):
                return None
            value = value[int(token)]
        elif get_is_object(value) and token in value:
            value = value[token]
        else:
            return None

    return value
