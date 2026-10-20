from __future__ import annotations

from dataclasses import dataclass
from typing import Literal

import polars as pl
from fairspec_metadata import (
    CellCheckError,
    CheckError,
    RowCheckError,
    TableCheckError,
    TableError,
)

from fairspec_table.actions.table.normalize import normalize_columns
from fairspec_table.models import SchemaMapping, Table, TableCheck
from fairspec_table.settings import NUMBER_COLUMN_NAME, QUERY_ENGINE


@dataclass
class _CheckItem:
    index: int
    check: TableCheck
    kind: Literal["row", "table", "invalid"]
    message: str = ""


def inspect_table_checks(
    table: Table,
    checks: list[TableCheck],
    *,
    mapping: SchemaMapping | None = None,
    max_errors: int,
) -> list[TableError]:
    """
    Check every row of the table against each condition, or the whole table when a
    condition reduces to one value; conditions see the values typed by the schema
    """
    errors: list[TableError] = []
    if not checks:
        return errors

    schema = _create_typed_table(table, mapping).collect_schema()
    items = [
        _infer_check_item(index, check, schema) for index, check in enumerate(checks)
    ]

    rows = _collect_failing_rows(
        table,
        [item for item in items if item.kind == "row"],
        mapping=mapping,
        max_errors=max_errors,
    )

    values = _collect_table_values(
        _create_typed_table(table, mapping),
        [item for item in items if item.kind == "table"],
    )

    for item in items:
        check = item.check
        check_name = _get_check_name(item.index)

        if item.kind == "invalid":
            errors.append(
                CheckError(type="check", checkName=check["name"], message=item.message)
            )
            continue

        if item.kind == "table":
            if values.get(check_name) is False:
                errors.append(
                    TableCheckError(
                        type="table/check",
                        checkName=check["name"],
                        message=check.get("message"),
                    )
                )
            continue

        column_name = check.get("columnName")
        for row in rows:
            if row[check_name] is not False:
                continue

            row_number = row[NUMBER_COLUMN_NAME]
            if column_name is None:
                errors.append(
                    RowCheckError(
                        type="row/check",
                        rowNumber=row_number,
                        checkName=check["name"],
                        message=check.get("message"),
                    )
                )
                continue

            cell = row[_get_cell_name(item.index)]
            errors.append(
                CellCheckError(
                    type="cell/check",
                    columnName=column_name,
                    rowNumber=row_number,
                    cell=str(cell if cell is not None else ""),
                    checkName=check["name"],
                    message=check.get("message"),
                )
            )

    return errors


# A condition is run on two typed rows of nulls: a row check returns both and an
# aggregate returns one, which would otherwise be broadcast to fail every row
def _infer_check_item(index: int, check: TableCheck, schema: pl.Schema) -> _CheckItem:
    column_name = check.get("columnName")
    if column_name is not None and column_name not in schema:
        message = f'Column "{column_name}" is not found'
        return _CheckItem(index, check, "invalid", message)

    probe_table = (
        pl.DataFrame({NUMBER_COLUMN_NAME: [1, 2]})
        .lazy()
        .with_columns(
            pl.lit(None).cast(dtype).alias(name)
            for name, dtype in schema.items()
            if name != NUMBER_COLUMN_NAME
        )
        .drop(NUMBER_COLUMN_NAME)
    )

    try:
        name = _get_check_name(index)
        frame: pl.DataFrame = probe_table.select(  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
            _create_condition(check).alias(name)
        ).collect(engine=QUERY_ENGINE)
    except pl.exceptions.PolarsError as error:
        return _CheckItem(index, check, "invalid", str(error).split("\n")[0])

    dtype = frame.schema[name]
    if dtype != pl.Boolean:
        message = f"Condition is {dtype.base_type().__name__} instead of Boolean"
        return _CheckItem(index, check, "invalid", message)

    return _CheckItem(index, check, "table" if frame.height == 1 else "row")


def _collect_failing_rows(
    table: Table,
    items: list[_CheckItem],
    *,
    mapping: SchemaMapping | None,
    max_errors: int,
) -> list[dict]:
    if not items:
        return []

    cell_items = [item for item in items if item.check.get("columnName") is not None]
    check_names = [_get_check_name(item.index) for item in items]
    check_table = _create_typed_table(table, mapping, cell_items).with_columns(
        _create_condition(item.check).alias(_get_check_name(item.index)) for item in items
    )

    frame: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        check_table.filter(
            pl.any_horizontal(
                pl.col(name).not_().fill_null(False) for name in check_names
            )
        )
        .select(
            pl.col(NUMBER_COLUMN_NAME),
            *(pl.col(_get_cell_name(item.index)) for item in cell_items),
            *(pl.col(name) for name in check_names),
        )
        .head(max_errors)
        .collect(engine=QUERY_ENGINE)
    )

    return frame.to_dicts()


def _collect_table_values(table: Table, items: list[_CheckItem]) -> dict:
    if not items:
        return {}

    frame: pl.DataFrame = table.select(  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        _create_condition(item.check).alias(_get_check_name(item.index)) for item in items
    ).collect(engine=QUERY_ENGINE)

    return frame.row(0, named=True)


def _create_typed_table(
    table: Table,
    mapping: SchemaMapping | None,
    cell_items: list[_CheckItem] | None = None,
) -> Table:
    source_names = (
        {column.name for column in mapping.source.columns} if mapping else set()
    )

    cell_columns: list[pl.Expr] = []
    for item in cell_items or []:
        column_name = item.check.get("columnName", "")
        is_sourced = not mapping or column_name in source_names
        cell_column = pl.col(column_name) if is_sourced else pl.lit(None)
        cell_columns.append(cell_column.alias(_get_cell_name(item.index)))

    numbered = table.with_row_index(NUMBER_COLUMN_NAME, 1)
    if not mapping:
        return numbered.with_columns(*cell_columns)

    return numbered.select(
        pl.col(NUMBER_COLUMN_NAME),
        *normalize_columns(mapping).values(),
        *cell_columns,
    )


def _create_condition(check: TableCheck) -> pl.Expr:
    condition = check["condition"]
    return pl.sql_expr(condition) if isinstance(condition, str) else condition


def _get_check_name(index: int) -> str:
    return f"fairspec:check:{index}"


def _get_cell_name(index: int) -> str:
    return f"fairspec:check-cell:{index}"
