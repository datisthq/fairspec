from __future__ import annotations

import json

from fairspec_metadata import (
    ArrayColumn,
    CellJsonError,
    CellTypeError,
    ColumnType,
    inspect_json,
)

from fairspec_table.models.table import Table
from fairspec_table.settings import QUERY_ENGINE, NUMBER_COLUMN_NAME


def inspect_array_column(
    column: ArrayColumn, table: Table
) -> list[CellTypeError | CellJsonError]:
    errors: list[CellTypeError | CellJsonError] = []

    import polars as pl

    frame: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .select(pl.col(NUMBER_COLUMN_NAME), pl.col(column.name).alias("source"))
        .collect(engine=QUERY_ENGINE)
    )

    constraint_json_schema = column.property.model_dump(exclude_none=True, by_alias=True)

    for row in frame.to_dicts():
        if row["source"] is None:
            continue

        target = None
        try:
            target = json.loads(row["source"])
        except (json.JSONDecodeError, TypeError):
            pass

        if target is None or not isinstance(target, list):
            errors.append(
                CellTypeError(
                    type="cell/type",
                    cell=str(row["source"]),
                    columnName=column.name,
                    columnType=ColumnType(column.type),
                    rowNumber=row[NUMBER_COLUMN_NAME],
                )
            )
            continue

        if constraint_json_schema:
            constraint_errors = inspect_json(target, json_schema=constraint_json_schema)
            for error in constraint_errors:
                errors.append(
                    CellJsonError(
                        type="cell/json",
                        cell=str(row["source"]),
                        columnName=column.name,
                        rowNumber=row[NUMBER_COLUMN_NAME],
                        message=error["message"],
                        jsonPointer=error["jsonPointer"],
                    )
                )

    return errors
