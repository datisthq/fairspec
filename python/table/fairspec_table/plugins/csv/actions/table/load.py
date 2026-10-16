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

    tables: list[Table] = []
    for path in paths:
        table = pl.scan_csv(path, **scan_options)  # type: ignore[arg-type]
        tables.append(table)

    result = pl.concat(tables)

    has_header = scan_options.get("has_header", True)
    column_names: list[str] | None = getattr(file_dialect, "columnNames", None)
    if not has_header and not column_names:
        result = result.rename(
            {
                name: name.replace("column_", "column")
                for name in result.collect_schema().names()
            }
        )

    header_rows = get_header_rows(file_dialect)  # type: ignore[arg-type]
    if len(header_rows) >= 2:
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

    options["skip_rows"] = header_rows[0] - 1 if header_rows else 0
    options["has_header"] = len(header_rows) > 0
    options["eol_char"] = getattr(file_dialect, "lineTerminator", None) or "\n"

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

    column_names: list[str] | None = getattr(file_dialect, "columnNames", None)
    if column_names:
        options["schema"] = {name: pl.String for name in column_names}
        options["has_header"] = False

    return options


def _dialect_has_only_format(dialect: FileDialect) -> bool:
    keys = {
        k for k in type(dialect).model_fields if getattr(dialect, k, None) is not None
    }
    meaningful = keys - {"format", "type", "title", "description"}
    return len(meaningful) == 0
