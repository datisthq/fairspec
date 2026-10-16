from __future__ import annotations

import json
from importlib.resources import files

from fairspec_metadata import CellTypeError, ColumnType, GeojsonColumn, inspect_json

from fairspec_table.models.table import Table
from fairspec_table.settings import QUERY_ENGINE, NUMBER_COLUMN_NAME


def inspect_geojson_column(column: GeojsonColumn, table: Table) -> list[CellTypeError]:
    errors: list[CellTypeError] = []

    import polars as pl

    frame: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .select(pl.col(NUMBER_COLUMN_NAME), pl.col(column.name).alias("source"))
        .collect(engine=QUERY_ENGINE)
    )

    type_json_schema = _load_geojson_schema()

    for row in frame.to_dicts():
        if row["source"] is None:
            continue

        target = None
        try:
            target = json.loads(row["source"])
        except (json.JSONDecodeError, TypeError):
            pass

        if target is None or not isinstance(target, dict):
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

        format_errors = inspect_json(target, json_schema=type_json_schema)
        if format_errors:
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


def _load_geojson_schema() -> dict[str, object]:
    schema_file = files("fairspec_table.schemas").joinpath("geojson.json")
    return json.loads(schema_file.read_text(encoding="utf-8"))
