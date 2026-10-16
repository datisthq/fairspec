from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import Resource, infer_file_dialect_format

if TYPE_CHECKING:
    from fairspec_metadata import FileDialect


def create_file_dialect_from_path_and_options(
    path: str,
    *,
    format: str | None = None,
    delimiter: str | None = None,
    line_terminator: str | None = None,
    quote_char: str | None = None,
    null_sequence: str | None = None,
    header_rows: str | None = None,
    header_join: str | None = None,
    comment_rows: str | None = None,
    comment_prefix: str | None = None,
    column_names: str | None = None,
    json_pointer: str | None = None,
    row_type: str | None = None,
    sheet_number: int | None = None,
    sheet_name: str | None = None,
    table_name: str | None = None,
) -> FileDialect | None:
    resolved_format = format or infer_file_dialect_format(Resource(data=path))

    parsed_header_rows = _parse_header_rows(header_rows) if header_rows else None
    parsed_comment_rows = _parse_int_list(comment_rows) if comment_rows else None
    parsed_column_names = column_names.split(",") if column_names else None

    if resolved_format in ("csv", "tsv"):
        file_dialect: dict[str, object] = {"format": resolved_format}

        if line_terminator:
            file_dialect["lineTerminator"] = line_terminator
        if null_sequence:
            file_dialect["nullSequence"] = null_sequence
        if parsed_header_rows is not None:
            file_dialect["headerRows"] = parsed_header_rows
        if header_join:
            file_dialect["headerJoin"] = header_join
        if parsed_comment_rows:
            file_dialect["commentRows"] = parsed_comment_rows
        if comment_prefix:
            file_dialect["commentPrefix"] = comment_prefix
        if parsed_column_names:
            file_dialect["columnNames"] = parsed_column_names

        if resolved_format == "csv":
            if delimiter:
                file_dialect["delimiter"] = delimiter
            if quote_char:
                file_dialect["quoteChar"] = quote_char

        return file_dialect  # type: ignore[return-value]

    if resolved_format in ("xlsx", "ods"):
        file_dialect = {"format": resolved_format}

        if sheet_number is not None:
            file_dialect["sheetNumber"] = sheet_number
        if sheet_name:
            file_dialect["sheetName"] = sheet_name
        if parsed_header_rows is not None:
            file_dialect["headerRows"] = parsed_header_rows
        if header_join:
            file_dialect["headerJoin"] = header_join
        if parsed_comment_rows:
            file_dialect["commentRows"] = parsed_comment_rows
        if comment_prefix:
            file_dialect["commentPrefix"] = comment_prefix

        return file_dialect  # type: ignore[return-value]

    if resolved_format in ("json", "jsonl"):
        file_dialect = {"format": resolved_format}

        if parsed_header_rows is not None:
            file_dialect["headerRows"] = parsed_header_rows
        if header_join:
            file_dialect["headerJoin"] = header_join
        if parsed_comment_rows:
            file_dialect["commentRows"] = parsed_comment_rows
        if comment_prefix:
            file_dialect["commentPrefix"] = comment_prefix
        if row_type:
            file_dialect["rowType"] = row_type

        if resolved_format == "json":
            if json_pointer:
                file_dialect["jsonPointer"] = json_pointer

        return file_dialect  # type: ignore[return-value]

    if resolved_format == "sqlite":
        file_dialect = {"format": resolved_format}

        if table_name:
            file_dialect["tableName"] = table_name

        return file_dialect  # type: ignore[return-value]

    return None


def _parse_header_rows(value: str) -> list[int] | bool:
    if value == "false":
        return False
    return [int(x) for x in value.split(",")]


def _parse_int_list(value: str) -> list[int]:
    return [int(x) for x in value.split(",")]
