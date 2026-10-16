from __future__ import annotations

import json
import math

import polars as pl
from fairspec_dataset.helpers.concurrency import iter_concurrent_chunks
from fairspec_metadata import TableError, get_columns
from fairspec_metadata import Column
from fairspec_metadata import ColumnMissingError
from fairspec_metadata import RowError
from fairspec_metadata import TableSchema
from pydantic import TypeAdapter

from fairspec_table.actions.column.inspect import inspect_column
from fairspec_table.helpers.schema import get_polars_schema
from fairspec_table.models import ColumnMapping, SchemaMapping, Table
from fairspec_table.settings import (
    ERROR_COLUMN_NAME,
    INSPECT_COLUMN_CONCURRENCY,
    NUMBER_COLUMN_NAME,
    QUERY_ENGINE,
)

from .checks.key import RowKeyCheck, create_row_key_checks

_row_error_adapter = TypeAdapter(RowError)


def inspect_table(
    table: Table,
    *,
    table_schema: TableSchema | None = None,
    sample_rows: int = 100,
    max_errors: int = 1000,
    concurrency: int | None = None,
) -> list[TableError]:
    errors: list[TableError] = []

    if table_schema:
        sample: pl.DataFrame = table.head(sample_rows).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        polars_schema = get_polars_schema(dict(sample.schema))
        mapping = SchemaMapping(source=polars_schema, target=table_schema)

        column_errors = _inspect_columns(
            mapping, table, max_errors=max_errors, concurrency=concurrency
        )
        errors.extend(column_errors)

        row_errors = _inspect_rows(
            mapping, table, max_errors=max_errors, concurrency=concurrency
        )
        errors.extend(row_errors)

    return errors[:max_errors]


def _inspect_columns(
    mapping: SchemaMapping,
    table: Table,
    *,
    max_errors: int,
    concurrency: int | None = None,
) -> list[TableError]:
    errors: list[TableError] = []
    columns = get_columns(mapping.target.model_dump())
    max_column_errors = math.ceil(max_errors / len(columns)) if columns else max_errors

    def inspect(column: Column) -> list[TableError]:
        polars_column = next(
            (pc for pc in mapping.source.columns if pc.name == column.name),
            None,
        )

        if not polars_column:
            is_required = mapping.target.allRequired or (
                mapping.target.required and column.name in mapping.target.required
            )
            if is_required:
                return [ColumnMissingError(type="column/missing", columnName=column.name)]
            return []

        column_mapping = ColumnMapping(source=polars_column, target=column)
        return inspect_column(column_mapping, table, max_errors=max_column_errors)

    for chunk in iter_concurrent_chunks(
        inspect, columns, concurrency=concurrency or INSPECT_COLUMN_CONCURRENCY
    ):
        for column_errors in chunk:
            errors.extend(column_errors)

        if len(errors) >= max_errors:
            break

    return errors


def _inspect_rows(
    mapping: SchemaMapping,
    table: Table,
    *,
    max_errors: int,
    concurrency: int | None = None,
) -> list[TableError]:
    errors: list[TableError] = []
    columns = get_columns(mapping.target.model_dump())
    max_row_errors = math.ceil(max_errors / len(columns)) if columns else max_errors

    def inspect(check: RowKeyCheck) -> list[TableError]:
        # A literal is broadcast to every row, so flag the error with a boolean rather
        # than a JSON template, which would materialize a full-length string column.
        # The check owns a single template, so it is read from the closure below.
        row_check_table = table.with_row_index(NUMBER_COLUMN_NAME, 1).with_columns(
            pl.when(check.is_error_expr)
            .then(pl.lit(True))
            .otherwise(pl.lit(None, dtype=pl.Boolean))
            .alias(ERROR_COLUMN_NAME)
        )

        row_check_frame: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
            row_check_table.filter(pl.col(ERROR_COLUMN_NAME).is_not_null())
            .head(max_row_errors)
            .collect(engine=QUERY_ENGINE)
        )

        check_errors: list[TableError] = []
        for row in row_check_frame.to_dicts():
            error_dict = json.loads(check.error_template)
            error_dict["rowNumber"] = row[NUMBER_COLUMN_NAME]
            check_errors.append(_row_error_adapter.validate_python(error_dict))

        return check_errors

    checks = create_row_key_checks(mapping)
    for chunk in iter_concurrent_chunks(inspect, checks, concurrency=concurrency):
        for check_errors in chunk:
            errors.extend(check_errors)

        if len(errors) >= max_errors:
            break

    return errors
