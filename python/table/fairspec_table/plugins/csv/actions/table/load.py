from __future__ import annotations

import os
from typing import TYPE_CHECKING, Unpack, cast

import polars as pl

from fairspec_dataset import prefetch_files
from fairspec_metadata import Resource, get_supported_file_dialect, resolve_table_schema

from fairspec_table.actions.table.file_dialect import (
    assert_part_headers,
    concat_part_tables,
    create_part_header_exception,
    join_header_rows,
    skip_comment_rows,
)
from fairspec_table.actions.table.normalize import normalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table
from fairspec_table.helpers.file_dialect import (
    assert_unique_column_names,
    get_header_rows,
    join_header_labels,
)
from fairspec_table.plugins.csv.actions.file_dialect.infer import infer_csv_file_dialect
from fairspec_table.plugins.csv.helpers.delimiter import restore_delimiter, swap_delimiter
from fairspec_table.plugins.csv.helpers.rows import clean_csv_rows
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

    is_headerless = not get_header_rows(file_dialect)  # type: ignore[arg-type]
    part_sizes = [os.path.getsize(path) for path in paths] if len(paths) > 1 else []
    empty_part = part_sizes.index(0) if 0 in part_sizes else -1
    if empty_part != -1 and not is_headerless:
        raise create_part_header_exception(empty_part + 1, [])
    # A headerless part with no bytes adds no records, and Polars refuses an empty file
    if empty_part != -1 and any(part_sizes):
        paths = [path for path, size in zip(paths, part_sizes, strict=True) if size]

    swapped = swap_delimiter(paths, file_dialect, max_bytes=max_bytes)
    if swapped:
        paths = swapped.paths
        file_dialect = swapped.file_dialect
    cleaned = (
        clean_csv_rows(paths, file_dialect, max_bytes=max_bytes)  # type: ignore[arg-type]
        if file_dialect
        else None
    )
    if cleaned:
        paths = cleaned.paths
        file_dialect = cleaned.file_dialect

    column_names: list[str] | None = getattr(file_dialect, "columnNames", None)
    header_rows = get_header_rows(file_dialect)  # type: ignore[arg-type]
    is_multi_header = len(header_rows) > 1 and not column_names
    is_own_header = len(paths) > 1 and not is_headerless and bool(column_names)

    # Header and comment rows count the rows of each part, so each part reads its own
    tables: list[Table] = []
    part_headers: list[list[str]] = []
    for path in paths:
        part_dialect = _renumber_prefixed_rows(path, file_dialect)
        scan_options = _get_scan_options(part_dialect)
        part_header_rows = get_header_rows(part_dialect)  # type: ignore[arg-type]
        # Compared before the part is read: joining its header rows reads it eagerly
        if is_own_header:
            part_headers.append(
                _read_own_header(path, scan_options, part_dialect)  # type: ignore[arg-type]
            )
            assert_part_headers(part_headers)
        if is_multi_header:
            scan_options["schema"] = _get_headerless_schema(
                path, scan_options, part_header_rows
            )
            # A first header row may be shorter than the schema
            scan_options["missing_columns"] = "insert"

        has_header = scan_options.get("has_header", True)
        if has_header:
            names = _read_header_names(path, scan_options)
            if not column_names:
                assert_unique_column_names(names)
        table = pl.scan_csv(path, **scan_options)  # type: ignore[arg-type]
        if has_header and column_names:
            table = _rename_header_columns(table, names, column_names)
        if swapped:
            table = restore_delimiter(table, swapped.swap)
        if len(part_header_rows) > 1:
            table = join_header_rows(table, part_dialect)  # type: ignore[arg-type]
        if getattr(part_dialect, "commentRows", None):
            table = skip_comment_rows(table, part_dialect)  # type: ignore[arg-type]
        tables.append(table)

    result = concat_part_tables(
        tables,
        file_dialect,  # type: ignore[arg-type]
        part_headers if is_own_header else None,
    )

    if is_headerless and not column_names:
        # By position, not from polars' own names: polars 2 numbers them from column_0
        result = result.rename(
            {
                name: f"column{index + 1}"
                for index, name in enumerate(result.collect_schema().names())
            }
        )

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
        "truncate_ragged_lines": False,
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
    # header are put in by position after the scan (`_rename_header_columns`)
    if column_names and not options["has_header"]:
        options["schema"] = {name: pl.String for name in column_names}
        options["missing_columns"] = "insert"

    return options


# Polars renames a repeated header name, so the names are read as a data row
def _read_header_names(path: str, options: dict[str, object]) -> list[str]:
    scan_options = {
        key: value
        for key, value in options.items()
        if key not in ("null_values", "schema", "missing_columns")
    }
    scan_options.update(has_header=False, truncate_ragged_lines=True)
    table = pl.scan_csv(path, **scan_options)  # type: ignore[arg-type]
    frame: pl.DataFrame = table.head(1).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
    cells = frame.row(0) if frame.height else ()
    return [cell if isinstance(cell, str) else "" for cell in cells]


# With columnNames the scan names the columns, so a part's own header rows are read
# as data for the multipart check
def _read_own_header(
    path: str,
    options: dict[str, object],
    file_dialect: CsvFileDialect | TsvFileDialect | None,
) -> list[str]:
    header_rows = get_header_rows(file_dialect)
    first_row = header_rows[0]
    height = header_rows[-1] - first_row + 1
    scan_options = {
        key: value
        for key, value in options.items()
        if key not in ("null_values", "schema")
    }
    # A first header row may be shorter than the rows below it
    scan_options.update(
        has_header=False, truncate_ragged_lines=True, missing_columns="insert"
    )
    # The widest header row sets the width, and every cell is read as text
    names = (
        pl.scan_csv(path, **{**scan_options, "infer_schema_length": height})  # type: ignore[arg-type]
        .collect_schema()
        .names()
    )
    scan_options["schema"] = {name: pl.String for name in names}
    table = pl.scan_csv(path, **scan_options)  # type: ignore[arg-type]
    frame: pl.DataFrame = table.head(height).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
    rows = frame.rows()
    header = [
        list(rows[number - first_row]) if number - first_row < len(rows) else []
        for number in header_rows
    ]
    if len(header) > 1:
        header_join = getattr(file_dialect, "headerJoin", None)
        return join_header_labels(header, " " if header_join is None else header_join)
    return [cell if isinstance(cell, str) else "" for cell in header[0]]


# The n-th name names the n-th column; fewer names than columns is an error, and more
# names add columns that are null in every row
def _rename_header_columns(
    table: Table, header_names: list[str], column_names: list[str]
) -> Table:
    if len(header_names) > len(column_names):
        raise Exception(
            f"The file has {len(header_names)} columns but columnNames has {len(column_names)}"
        )
    names = table.collect_schema().names()
    return table.select(
        *(pl.col(name).alias(column_names[index]) for index, name in enumerate(names)),
        *(pl.lit(None, pl.String).alias(name) for name in column_names[len(names) :]),
    )


# Header rows are joined from the data, so every column is read as text and the
# widest header row sets the width: a first header row may be shorter, and a data
# row wider than every header row is a long row
def _get_headerless_schema(
    path: str, options: dict[str, object], header_rows: list[int]
) -> dict[str, object]:
    scan_options = {
        **options,
        "infer_schema_length": max(header_rows) - min(header_rows) + 1,
        "truncate_ragged_lines": True,
    }
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
        if key not in ("null_values", "comment_prefix", "schema", "missing_columns")
    }
    table = pl.scan_csv(
        path,
        **{
            **scan_options,
            "skip_rows": 0,
            "has_header": False,
            "truncate_ragged_lines": True,
        },  # type: ignore[arg-type]
    )
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
