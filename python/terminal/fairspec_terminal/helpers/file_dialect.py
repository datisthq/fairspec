from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import Resource, infer_file_dialect_format

if TYPE_CHECKING:
    from fairspec_metadata import FileDialect

    from fairspec_terminal.params import DialectParams


def create_file_dialect_from_path_and_options(
    path: str, params: DialectParams
) -> FileDialect | None:
    resolved_format = params.format or infer_file_dialect_format(Resource(data=path))

    if resolved_format in ("csv", "tsv"):
        file_dialect: dict[str, object] = {"format": resolved_format}

        if params.line_terminator:
            file_dialect["lineTerminator"] = params.line_terminator
        if params.null_sequence:
            file_dialect["nullSequence"] = params.null_sequence
        if params.header_rows is not None:
            file_dialect["headerRows"] = params.header_rows
        if params.header_join:
            file_dialect["headerJoin"] = params.header_join
        if params.comment_rows:
            file_dialect["commentRows"] = params.comment_rows
        if params.comment_prefix:
            file_dialect["commentPrefix"] = params.comment_prefix
        if params.column_names:
            file_dialect["columnNames"] = params.column_names

        if resolved_format == "csv":
            if params.delimiter:
                file_dialect["delimiter"] = params.delimiter
            if params.quote_char:
                file_dialect["quoteChar"] = params.quote_char

        return file_dialect  # type: ignore[return-value]

    if resolved_format in ("xlsx", "ods"):
        file_dialect = {"format": resolved_format}

        if params.sheet_number is not None:
            file_dialect["sheetNumber"] = params.sheet_number
        if params.sheet_name:
            file_dialect["sheetName"] = params.sheet_name
        if params.header_rows is not None:
            file_dialect["headerRows"] = params.header_rows
        if params.header_join:
            file_dialect["headerJoin"] = params.header_join
        if params.comment_rows:
            file_dialect["commentRows"] = params.comment_rows
        if params.comment_prefix:
            file_dialect["commentPrefix"] = params.comment_prefix

        return file_dialect  # type: ignore[return-value]

    if resolved_format in ("json", "jsonl"):
        file_dialect = {"format": resolved_format}

        if params.header_rows is not None:
            file_dialect["headerRows"] = params.header_rows
        if params.header_join:
            file_dialect["headerJoin"] = params.header_join
        if params.comment_rows:
            file_dialect["commentRows"] = params.comment_rows
        if params.comment_prefix:
            file_dialect["commentPrefix"] = params.comment_prefix
        if params.row_type:
            file_dialect["rowType"] = params.row_type

        if resolved_format == "json":
            if params.json_pointer:
                file_dialect["jsonPointer"] = params.json_pointer

        return file_dialect  # type: ignore[return-value]

    if resolved_format == "sqlite":
        file_dialect = {"format": resolved_format}

        if params.table_name:
            file_dialect["tableName"] = params.table_name

        return file_dialect  # type: ignore[return-value]

    return None
