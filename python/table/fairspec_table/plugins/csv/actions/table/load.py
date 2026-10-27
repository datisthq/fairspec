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
from fairspec_table.settings import NUMBER_COLUMN_NAME, QUERY_ENGINE

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

    if not resource.fileDialect:
        inferred = infer_csv_file_dialect(
            Resource(data=paths[0], fileDialect=cast("FileDialect", file_dialect))
        )
        if inferred:
            file_dialect = inferred
    file_dialect = _renumber_prefixed_rows(paths[0], file_dialect)

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


# Polars drops prefixed rows before it skips or numbers any, while headerRows and
# commentRows count every row of the file: renumber them past the prefixed rows
def _renumber_prefixed_rows(
    path: str,
    file_dialect: CsvFileDialect | TsvFileDialect | FileDialect | None,
) -> CsvFileDialect | TsvFileDialect | FileDialect | None:
    header_rows = getattr(file_dialect, "headerRows", None)
    comment_rows: list[int] | None = getattr(file_dialect, "commentRows", None)
    comment_prefix: str | None = getattr(file_dialect, "commentPrefix", None)
    header_numbers = header_rows if isinstance(header_rows, list) else []
    last_row = max([0, *header_numbers, *(comment_rows or [])])
    if file_dialect is None or not comment_prefix or last_row < 2:
        return file_dialect

    scan_options = {
        key: value
        for key, value in _get_scan_options(file_dialect).items()
        if key not in ("null_values", "comment_prefix", "schema", "new_columns")
    }
    table = pl.scan_csv(path, **{**scan_options, "skip_rows": 0, "has_header": False})  # type: ignore[arg-type]
    first = pl.col(table.collect_schema().names()[0])
    frame: pl.DataFrame = (
        table.head(last_row)
        .with_row_index(NUMBER_COLUMN_NAME, 1)
        .filter(first.str.starts_with(comment_prefix))
        .select(NUMBER_COLUMN_NAME)
        .collect(engine=QUERY_ENGINE)
    )  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
    prefixed_rows: list[int] = frame.to_series().to_list()

    def renumber_row(row: int) -> int:
        return row - sum(1 for prefixed in prefixed_rows if prefixed < row)

    update: dict[str, object] = {}
    if header_numbers:
        update["headerRows"] = list(dict.fromkeys(map(renumber_row, header_numbers)))
    if comment_rows:
        update["commentRows"] = [
            renumber_row(row) for row in comment_rows if row not in prefixed_rows
        ]
    return file_dialect.model_copy(update=update)
