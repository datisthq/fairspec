from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

import polars as pl

from fairspec_dataset import prefetch_files
from fairspec_metadata import (
    OdsFileDialect,
    Resource,
    XlsxFileDialect,
    get_supported_file_dialect,
    resolve_table_schema,
)

from fairspec_table.actions.table.normalize import normalize_table
from fairspec_table.actions.table_schema.infer import infer_table_schema_from_table
from fairspec_table.plugins.xlsx.actions.file_dialect.infer import (
    infer_xlsx_file_dialect,
)
from fairspec_table.plugins.xlsx.helpers.sheet import read_sheet_rows
from fairspec_table.plugins.xlsx.helpers.table import create_table_from_sheet_rows

if TYPE_CHECKING:
    from fairspec_table.models.table import LoadTableOptions, Table


def load_xlsx_table(resource: Resource, **options: Unpack[LoadTableOptions]) -> Table:
    file_dialect = get_supported_file_dialect(resource, ["xlsx", "ods"])
    if not file_dialect:
        raise Exception("Resource data is not compatible")

    paths = prefetch_files(resource, concurrency=options.get("concurrency"))
    if not paths:
        raise Exception("Resource path is not defined")

    if not resource.fileDialect:
        inferred = infer_xlsx_file_dialect(resource.model_copy(update={"data": paths[0]}))
        if inferred:
            file_dialect = inferred

    if not isinstance(file_dialect, XlsxFileDialect | OdsFileDialect):
        raise Exception("Resource data is not compatible")

    tables: list[Table] = []
    for path in paths:
        rows = read_sheet_rows(path, file_dialect)
        if rows is not None:
            tables.append(create_table_from_sheet_rows(rows, file_dialect))

    result = pl.concat(tables)

    if not options.get("denormalized"):
        table_schema = resolve_table_schema(resource.tableSchema)
        if not table_schema:
            table_schema = infer_table_schema_from_table(result, **options)
        result = normalize_table(result, table_schema)

    return result
