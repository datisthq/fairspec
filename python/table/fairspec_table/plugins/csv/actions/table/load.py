from __future__ import annotations

from typing import TYPE_CHECKING, Unpack, cast

import polars as pl

from fairspec_dataset import prefetch_files
from fairspec_metadata import Resource, get_supported_file_dialect, resolve_table_schema

from fairspec_table.actions.table.file_dialect import (
    join_header_rows,
    skip_comment_rows,
)
from fairspec_table.actions.table.normalize import normalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table
from fairspec_table.helpers.file_dialect import get_header_rows
from fairspec_table.plugins.csv.actions.file_dialect.infer import infer_csv_file_dialect

if TYPE_CHECKING:
    from fairspec_metadata import CsvFileDialect, TsvFileDialect
    from fairspec_metadata.models.file_dialect.file_dialect import FileDialect

    from fairspec_table.models.table import LoadTableOptions, Table


def load_csv_table(resource: Resource, **options: Unpack[LoadTableOptions]) -> Table:
    file_dialect = get_supported_file_dialect(resource, ["csv", "tsv"])
    if not file_dialect:
        raise Exception("Resource data is not compatible")

    max_bytes = options.get("previewBytes")
    paths = prefetch_files(
        resource, max_bytes=max_bytes, concurrency=options.get("concurrency")
    )
    if not paths:
        raise Exception("Resource path is not defined")

    if _dialect_has_only_format(file_dialect):
        inferred = infer_csv_file_dialect(
            Resource(data=paths[0], fileDialect=cast("FileDialect", file_dialect))
        )
        if inferred:
            file_dialect = inferred

    scan_options = _get_scan_options(file_dialect)
    column_names: list[str] | None = getattr(file_dialect, "columnNames", None)
    header_rows = get_header_rows(file_dialect)  # type: ignore[arg-type]
    is_multi_header = len(header_rows) > 1 and not column_names
    if is_multi_header:
        scan_options["schema"] = _get_headerless_schema(paths[0], scan_options)
        # Polars 2 sizes a headerless file by its first line, which may be the shorter
        # first header row; inserting keeps the wider rows' cells
        scan_options["missing_columns"] = "insert"

    tables: list[Table] = []
    for path in paths:
        table = pl.scan_csv(path, **scan_options)  # type: ignore[arg-type]
        tables.append(table)

    result = pl.concat(tables)

    has_header = scan_options.get("has_header", True)
    if not has_header and not column_names and not is_multi_header:
        # By position, not from polars' own names: polars 2 numbers them from column_0
        result = result.rename(
            {
                name: f"column{index + 1}"
                for index, name in enumerate(result.collect_schema().names())
            }
        )

    if len(header_rows) > 1:
        result = join_header_rows(result, file_dialect)  # type: ignore[arg-type]
    if getattr(file_dialect, "commentRows", None):
        result = skip_comment_rows(result, file_dialect)  # type: ignore[arg-type]

    if not options.get("denormalized"):
        table_schema = resolve_table_schema(resource.tableSchema)
        if not table_schema:
            table_schema = infer_table_schema_from_table(result, **options)
        result = normalize_table(result, table_schema)

    return result


def _get_scan_options(
    file_dialect: CsvFileDialect | TsvFileDialect | FileDialect | None,
) -> dict[str, object]:
    header_rows = get_header_rows(file_dialect)  # type: ignore[arg-type]

    options: dict[str, object] = {
        "infer_schema_length": 0,
        "truncate_ragged_lines": True,
    }

    column_names: list[str] | None = getattr(file_dialect, "columnNames", None)

    options["skip_rows"] = header_rows[0] - 1 if header_rows else 0
    options["has_header"] = len(header_rows) == 1 or (
        len(header_rows) > 1 and bool(column_names)
    )
    # Polars takes a single eol character and drops the "\r" before a "\n" itself
    options["eol_char"] = (getattr(file_dialect, "lineTerminator", None) or "\n")[-1]

    is_csv = getattr(file_dialect, "format", "csv") == "csv"
    if is_csv:
        options["separator"] = getattr(file_dialect, "delimiter", None) or ","
        options["quote_char"] = getattr(file_dialect, "quoteChar", None) or '"'
    else:
        options["separator"] = "\t"
        options["quote_char"] = None

    null_sequence = getattr(file_dialect, "nullSequence", None)
    if null_sequence is not None:
        options["null_values"] = null_sequence

    comment_prefix = getattr(file_dialect, "commentPrefix", None)
    if comment_prefix is not None:
        options["comment_prefix"] = comment_prefix

    # Polars 2 matches a schema against the header row by name, so names that replace a
    # header go in by position through new_columns
    if column_names and options["has_header"]:
        options["new_columns"] = column_names
    elif column_names:
        options["schema"] = {name: pl.String for name in column_names}

    return options


# Header rows are joined from the data, so every column is read as text and the
# widest of the first rows sets the width: a first header row may be shorter
def _get_headerless_schema(path: str, options: dict[str, object]) -> dict[str, object]:
    scan_options = {**options, "infer_schema_length": 100}
    names = pl.scan_csv(path, **scan_options).collect_schema().names()  # type: ignore[arg-type]
    return {name: pl.String for name in names}


def _dialect_has_only_format(dialect: FileDialect) -> bool:
    keys = {
        k for k in type(dialect).model_fields if getattr(dialect, k, None) is not None
    }
    meaningful = keys - {"format", "type", "title", "description"}
    return len(meaningful) == 0
