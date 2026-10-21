from __future__ import annotations

import json
import math

import polars as pl
from fairspec_dataset.helpers.concurrency import iter_concurrent_chunks
from fairspec_metadata import TableError, get_columns
from fairspec_metadata import ColumnExtraError, ColumnMissingError
from fairspec_metadata import RowError
from fairspec_metadata import TableSchema
from pydantic import TypeAdapter

from fairspec_table.actions.check.inspect import inspect_table_checks
from fairspec_table.actions.column.inspect import inspect_columns
from fairspec_table.helpers.schema import get_polars_schema
from fairspec_table.models import ColumnMapping, SchemaMapping, Table, TableCheck
from fairspec_table.settings import (
    ERROR_COLUMN_NAME,
    NUMBER_COLUMN_NAME,
    QUERY_ENGINE,
)

from ._shared.helpers import merge_missing_values
from .checks.key import (
    KeyHashProbe,
    RowKeyCheck,
    create_key_hash_probe,
    create_row_key_checks,
)

_row_error_adapter = TypeAdapter(RowError)


def inspect_table(
    table: Table,
    *,
    table_schema: TableSchema | None = None,
    checks: list[TableCheck] | None = None,
    sample_rows: int = 100,
    max_errors: int = 1000,
    concurrency: int | None = None,
) -> list[TableError]:
    errors: list[TableError] = []
    mapping: SchemaMapping | None = None

    if table_schema:
        sample: pl.DataFrame = table.head(sample_rows).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        polars_schema = get_polars_schema(dict(sample.schema))
        mapping = SchemaMapping(source=polars_schema, target=table_schema)

        key_checks = create_row_key_checks(mapping)
        key_probes = [create_key_hash_probe(check) for check in key_checks]
        column_errors = _inspect_columns(
            mapping,
            table,
            max_errors=max_errors,
            flag_exprs=[probe.flag_expr for probe in key_probes],
        )
        errors.extend(column_errors)

        # The cell checks stop reading at max_errors, leaving the key hashes partial, but
        # only once their errors fill the report, which then has no room for row errors
        if len(errors) < max_errors:
            row_errors = _inspect_rows(
                mapping,
                table,
                key_checks=key_checks,
                key_probes=key_probes,
                max_errors=max_errors,
                concurrency=concurrency,
            )
            errors.extend(row_errors)

    if checks and len(errors) < max_errors:
        check_errors = inspect_table_checks(
            table, checks, mapping=mapping, max_errors=max_errors
        )
        errors.extend(check_errors)

    return errors[:max_errors]


def _inspect_columns(
    mapping: SchemaMapping,
    table: Table,
    *,
    max_errors: int,
    flag_exprs: list[pl.Expr],
) -> list[TableError]:
    errors: list[TableError] = []
    columns = get_columns(mapping.target.model_dump())

    missing_errors: dict[str, list[TableError]] = {}
    column_mappings: list[ColumnMapping] = []
    for column in columns:
        polars_column = next(
            (pc for pc in mapping.source.columns if pc.name == column.name),
            None,
        )

        if not polars_column:
            is_required = mapping.target.allRequired or (
                mapping.target.required and column.name in mapping.target.required
            )
            missing_errors[column.name] = (
                [ColumnMissingError(type="column/missing", columnName=column.name)]
                if is_required
                else []
            )
            continue

        merged_column = merge_missing_values(column, mapping.target)
        column_mappings.append(ColumnMapping(source=polars_column, target=merged_column))

    cell_errors = inspect_columns(
        column_mappings, table, max_errors=max_errors, flag_exprs=flag_exprs
    )
    for column in columns:
        errors.extend(missing_errors.get(column.name) or cell_errors.get(column.name, []))

    if mapping.target.additionalProperties is False:
        names = {column.name for column in columns}
        for polars_column in mapping.source.columns:
            if polars_column.name not in names:
                errors.append(
                    ColumnExtraError(type="column/extra", columnName=polars_column.name)
                )

    return sorted(errors, key=lambda error: not error.type.startswith("column/"))


def _inspect_rows(
    mapping: SchemaMapping,
    table: Table,
    *,
    key_checks: list[RowKeyCheck],
    key_probes: list[KeyHashProbe],
    max_errors: int,
    concurrency: int | None = None,
) -> list[TableError]:
    errors: list[TableError] = []
    columns = get_columns(mapping.target.model_dump())
    max_row_errors = math.ceil(max_errors / len(columns)) if columns else max_errors

    def inspect(entry: tuple[RowKeyCheck, KeyHashProbe]) -> list[TableError]:
        check, probe = entry
        if not probe.get_has_duplicate_key():
            return []

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

    entries = list(zip(key_checks, key_probes))
    for chunk in iter_concurrent_chunks(inspect, entries, concurrency=concurrency):
        for check_errors in chunk:
            errors.extend(check_errors)

        if len(errors) >= max_errors:
            break

    return errors
