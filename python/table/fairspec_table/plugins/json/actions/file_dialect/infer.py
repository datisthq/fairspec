from __future__ import annotations

from typing import TYPE_CHECKING, cast

from fairspec_dataset import load_file, load_file_stream
from fairspec_metadata import get_data_first_path, get_supported_file_dialect
from fairspec_metadata.models.file_dialect.common import RowType
from fairspec_metadata.models.file_dialect.json import JsonFileDialect
from fairspec_metadata.models.file_dialect.jsonl import JsonlFileDialect

from fairspec_table.plugins.json.actions.buffer.decode import decode_json_buffer
from fairspec_table.utils.sniffer.sniffer import Sniffer

if TYPE_CHECKING:
    from fairspec_metadata import Resource

    from fairspec_table.models.table import LoadTableOptions


def infer_json_file_dialect(
    resource: Resource,
    options: LoadTableOptions | None = None,
) -> JsonFileDialect | JsonlFileDialect | None:
    data_path = get_data_first_path(resource)
    if not data_path:
        return None

    dialect = get_supported_file_dialect(resource, ["json", "jsonl"])
    if not dialect:
        return None

    format: str = getattr(dialect, "format", None) or (
        dialect.get("format") if isinstance(dialect, dict) else None
    )  # type: ignore[union-attr]

    try:
        if format == "json":
            json_buffer = load_file(data_path)
        else:
            stream = load_file_stream(data_path, max_bytes=10000)
            json_buffer = stream.read()
    except Exception:
        if format == "json":
            return JsonFileDialect()
        return JsonlFileDialect()

    try:
        parsed = decode_json_buffer(json_buffer, is_lines=format == "jsonl")
    except Exception:
        if format == "json":
            return JsonFileDialect()
        return JsonlFileDialect()

    json_pointer: str | None = None
    row_type: RowType | None = None
    header_rows: list[int] | bool | None = None

    data = parsed
    if not isinstance(data, list):
        if isinstance(data, dict):
            for key, value in data.items():
                if isinstance(value, list):
                    if format == "json":
                        json_pointer = str(key)
                    data = value
                    break

    if not isinstance(data, list) or len(data) == 0:
        if format == "json":
            return _build_json_dialect(json_pointer, row_type, header_rows)
        return _build_jsonl_dialect(row_type, header_rows)

    first_element = data[0]

    if isinstance(first_element, list):
        row_type = RowType.array
    elif isinstance(first_element, dict):
        row_type = RowType.object
    else:
        if format == "json":
            return _build_json_dialect(json_pointer, row_type, header_rows)
        return _build_jsonl_dialect(row_type, header_rows)

    if row_type == RowType.array:
        sample_rows = 100
        rows = cast("list[list[object]]", data[:sample_rows])

        sniffer = Sniffer()
        try:
            detection = sniffer.sniff_rows(rows)
        except Exception:
            if format == "json":
                return _build_json_dialect(json_pointer, row_type, header_rows)
            return _build_jsonl_dialect(row_type, header_rows)

        if detection.dialect.header.has_header_row:
            header_rows = [detection.dialect.header.num_preamble_rows + 1]
        elif detection.num_fields > 0:
            header_rows = False

    if format == "json":
        return _build_json_dialect(json_pointer, row_type, header_rows)
    return _build_jsonl_dialect(row_type, header_rows)


def _build_json_dialect(
    json_pointer: str | None,
    row_type: RowType | None,
    header_rows: list[int] | bool | None,
) -> JsonFileDialect:
    kwargs: dict[str, object] = {}
    if json_pointer is not None:
        kwargs["jsonPointer"] = json_pointer
    if row_type is not None:
        kwargs["rowType"] = row_type
    if header_rows is not None:
        kwargs["headerRows"] = header_rows
    return JsonFileDialect(**kwargs)  # type: ignore[arg-type]


def _build_jsonl_dialect(
    row_type: RowType | None,
    header_rows: list[int] | bool | None,
) -> JsonlFileDialect:
    kwargs: dict[str, object] = {}
    if row_type is not None:
        kwargs["rowType"] = row_type
    if header_rows is not None:
        kwargs["headerRows"] = header_rows
    return JsonlFileDialect(**kwargs)  # type: ignore[arg-type]
