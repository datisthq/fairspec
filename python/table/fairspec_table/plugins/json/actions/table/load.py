from __future__ import annotations

from typing import TYPE_CHECKING, Unpack, cast

import polars as pl

from fairspec_dataset import load_file, prefetch_files
from fairspec_metadata import Resource, get_supported_file_dialect, resolve_table_schema

from fairspec_table.actions.table.normalize import normalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table
from fairspec_table.plugins.json.actions.buffer.decode import decode_json_buffer
from fairspec_table.plugins.json.actions.file_dialect.infer import (
    infer_json_file_dialect,
)

if TYPE_CHECKING:
    from fairspec_metadata import JsonFileDialect, JsonlFileDialect
    from fairspec_metadata.models.file_dialect.file_dialect import FileDialect

    from fairspec_table.models.table import LoadTableOptions, Table


def load_json_table(resource: Resource, **options: Unpack[LoadTableOptions]) -> Table:
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
        data: object = decode_json_buffer(buffer, is_lines=is_lines)
        if not is_default:
            data = _process_data(data, file_dialect)
        table = pl.DataFrame(cast("list[dict[str, object]]", data)).lazy()
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


def _process_data(
    data: object,
    dialect: JsonFileDialect | JsonlFileDialect | FileDialect,
) -> list[dict[str, object]]:
    if getattr(dialect, "format", None) == "json" and getattr(
        dialect, "jsonPointer", None
    ):
        json_pointer: str = getattr(dialect, "jsonPointer")
        assert isinstance(data, dict)
        data = data[json_pointer]

    if getattr(dialect, "rowType", None) == "array":
        assert isinstance(data, list)
        keys = cast("list[str]", data[0])
        data = [dict(zip(keys, cast("list[object]", row))) for row in data[1:]]

    column_names: list[str] | None = getattr(dialect, "columnNames", None)
    if column_names:
        assert isinstance(data, list)
        data = [
            {name: cast("dict[str, object]", row)[name] for name in column_names}
            for row in data
        ]

    return cast("list[dict[str, object]]", data)
