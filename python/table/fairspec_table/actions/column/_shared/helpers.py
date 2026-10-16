from __future__ import annotations

import json
from typing import Callable, Union, cast

import polars as pl

from fairspec_metadata import (
    ArrayColumn,
    CellJsonError,
    CellTypeError,
    DurationColumn,
    GeojsonColumn,
    ObjectColumn,
    TopojsonColumn,
    WkbColumn,
    WktColumn,
    inspect_json,
)
from fairspec_metadata import ColumnType
from fairspec_metadata import CellError

from fairspec_table.helpers import get_is_object
from fairspec_table.models import Table
from fairspec_table.settings import QUERY_ENGINE, NUMBER_COLUMN_NAME


def inspect_text_column(
    column: Union[DurationColumn, WktColumn, WkbColumn],
    table: Table,
    *,
    parse: Callable[[str], object],
) -> list[CellError]:
    errors: list[CellError] = []

    frame = cast(
        pl.DataFrame,
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .select(pl.col(NUMBER_COLUMN_NAME), pl.col(column.name).alias("source"))
        .collect(engine=QUERY_ENGINE),
    )

    for row in frame.to_dicts():
        if row["source"] is None:
            continue

        target = None
        try:
            target = parse(row["source"])
        except Exception:
            pass

        if not target:
            errors.append(
                CellTypeError(
                    type="cell/type",
                    cell=str(row["source"]),
                    columnName=column.name,
                    columnType=ColumnType(column.type),
                    rowNumber=row[NUMBER_COLUMN_NAME],
                )
            )

    return errors


def inspect_json_column(
    column: Union[ArrayColumn, ObjectColumn, GeojsonColumn, TopojsonColumn],
    table: Table,
    *,
    type_json_schema: dict[str, object] | None = None,
) -> list[CellError]:
    errors: list[CellError] = []

    column_type = ColumnType(column.type)
    constraint_json_schema = column.property.model_dump(exclude_none=True, by_alias=True)

    frame = cast(
        pl.DataFrame,
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .select(pl.col(NUMBER_COLUMN_NAME), pl.col(column.name).alias("source"))
        .collect(engine=QUERY_ENGINE),
    )

    for row in frame.to_dicts():
        if row["source"] is None:
            continue

        target = None
        check_compat: Callable[[object], bool] = (
            (lambda v: isinstance(v, list)) if column.type == "array" else get_is_object
        )

        try:
            target = json.loads(row["source"])
        except Exception:
            pass

        if not target or not check_compat(target):
            errors.append(
                CellTypeError(
                    type="cell/type",
                    cell=str(row["source"]),
                    columnName=column.name,
                    columnType=column_type,
                    rowNumber=row[NUMBER_COLUMN_NAME],
                )
            )
            continue

        if type_json_schema:
            format_errors = inspect_json(target, json_schema=type_json_schema)

            if format_errors:
                errors.append(
                    CellTypeError(
                        type="cell/type",
                        cell=str(row["source"]),
                        columnName=column.name,
                        columnType=column_type,
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
