from __future__ import annotations

from typing import TYPE_CHECKING, Unpack, cast

import polars as pl

from fairspec_dataset import assert_local_path_vacant, save_file
from fairspec_metadata import Resource, TableSchema, get_supported_file_dialect

from fairspec_table.actions.table.denormalize import denormalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table
from fairspec_table.plugins.json.actions.buffer.decode import decode_json_buffer
from fairspec_table.plugins.json.actions.buffer.encode import encode_json_buffer
from fairspec_table.plugins.json.settings import NATIVE_TYPES
from fairspec_table.settings import QUERY_ENGINE

if TYPE_CHECKING:
    from fairspec_metadata import JsonFileDialect, JsonlFileDialect

    from fairspec_table.models.table import SaveTableOptions, Table


def save_json_table(table: Table, **options: Unpack[SaveTableOptions]) -> str:
    path = options["path"]

    if not options.get("overwrite"):
        assert_local_path_vacant(path)

    resource = Resource(data=path, fileDialect=options.get("fileDialect"))
    file_dialect = get_supported_file_dialect(resource, ["json", "jsonl"])
    if not file_dialect:
        raise Exception("Saving options is not compatible")

    is_lines = getattr(file_dialect, "format", None) == "jsonl"

    table_schema = options.get("tableSchema")
    if not isinstance(table_schema, TableSchema):
        table_schema = infer_table_schema_from_table(table, **options, keepStrings=True)

    table = denormalize_table(table, table_schema, nativeTypes=NATIVE_TYPES)

    frame = cast("pl.DataFrame", table.collect(engine=QUERY_ENGINE))
    if is_lines:
        text = frame.write_ndjson()
    else:
        text = frame.write_json()

    data = decode_json_buffer(text.encode("utf-8"), is_lines=is_lines)

    data = _process_data(data, file_dialect)

    buffer = encode_json_buffer(data, is_lines=is_lines)
    save_file(path, buffer, overwrite=bool(options.get("overwrite")))

    return path


def _process_data(
    records: object,
    dialect: JsonFileDialect | JsonlFileDialect | object,
) -> object:
    data: object = records

    column_names: list[str] | None = getattr(dialect, "columnNames", None)
    if column_names:
        assert isinstance(data, list)
        data = [
            {name: cast("dict[str, object]", row)[name] for name in column_names}
            for row in data
        ]

    if getattr(dialect, "rowType", None) == "array":
        assert isinstance(data, list)
        names: list[str] = column_names or list(cast("dict[str, object]", data[0]).keys())
        data = [
            names,
            *[[cast("dict[str, object]", row)[name] for name in names] for row in data],
        ]

    if getattr(dialect, "format", None) == "json":
        json_pointer: str | None = getattr(dialect, "jsonPointer", None)
        if json_pointer:
            data = {json_pointer: data}

    return data
