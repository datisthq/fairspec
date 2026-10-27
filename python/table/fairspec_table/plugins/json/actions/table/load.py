from __future__ import annotations

import io
from typing import TYPE_CHECKING, Unpack, cast

import polars as pl

from fairspec_dataset import load_file, load_file_stream, prefetch_files
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

    if not resource.fileDialect:
        inferred = infer_json_file_dialect(
            Resource(data=paths[0], fileDialect=cast("FileDialect", file_dialect))
        )
        if inferred:
            file_dialect = inferred

    is_lines = getattr(file_dialect, "format", None) == "jsonl"
    is_default = _is_default_dialect(file_dialect)

    tables: list[Table] = []
    for path in paths:
        if is_lines and is_default and not _get_is_first_item_array(path):
            table = pl.scan_ndjson(path)
            tables.append(table)
            continue

        buffer = load_file(path)
        frame = _read_object_array(buffer) if not is_lines and is_default else None
        if frame is not None:
            tables.append(frame.lazy())
            continue

        records = _get_records_from_json(
            decode_json_buffer(buffer, is_lines=is_lines), file_dialect
        )
        if records is None:
            return None
        table = _create_records_frame(records, file_dialect).lazy()
        tables.append(table)

    result = pl.concat(tables)

    if not options.get("denormalized"):
        table_schema = resolve_table_schema(resource.tableSchema)
        if not table_schema:
            table_schema = infer_table_schema_from_table(result, **options)
        result = normalize_table(result, table_schema)

    return result


def _is_default_dialect(dialect: FileDialect) -> bool:
    for key in type(dialect).model_fields:
        if key in ("format", "type", "title", "description"):
            continue
        value = getattr(dialect, key, None)
        if key == "rowType" and value == RowType.object:
            continue
        if value is not None:
            return False
    return True


# An empty array has no records to take the columns from
def _create_records_frame(
    records: list[DataRecord], dialect: FileDialect
) -> pl.DataFrame:
    column_names: list[str] | None = getattr(dialect, "columnNames", None)
    if records or not column_names:
        return pl.DataFrame(records)
    return pl.DataFrame(schema={name: pl.String for name in column_names})


# Polars reads only object lines, while a default dialect takes array rows when
# the first item is an array
def _get_is_first_item_array(path: str) -> bool:
    with load_file_stream(path) as stream:
        while chunk := stream.read(65536):
            index = _skip_json_whitespace(chunk, 0)
            if index < len(chunk):
                return chunk[index : index + 1] == b"["
    return False


# A default dialect still admits a top-level object (not a table) or array rows,
# so only a document opening with `[{` is read natively, and a refusal falls back
def _read_object_array(buffer: bytes) -> pl.DataFrame | None:
    start = _skip_json_whitespace(buffer, 0)
    if buffer[start : start + 1] != b"[":
        return None
    second = _skip_json_whitespace(buffer, start + 1)
    if buffer[second : second + 1] != b"{":
        return None

    try:
        return pl.read_json(io.BytesIO(buffer), infer_schema_length=None)
    except pl.exceptions.PolarsError:
        return None


def _skip_json_whitespace(buffer: bytes, index: int) -> int:
    while buffer[index : index + 1] in (b" ", b"\t", b"\n", b"\r"):
        index += 1
    return index


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
