from __future__ import annotations

import shapely

from fairspec_metadata import CellTypeError, ColumnType, WkbColumn

from fairspec_table.models.table import Table
from fairspec_table.settings import QUERY_ENGINE, NUMBER_COLUMN_NAME


def inspect_wkb_column(column: WkbColumn, table: Table) -> list[CellTypeError]:
    errors: list[CellTypeError] = []

    import polars as pl

    frame: pl.DataFrame = (  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        table.with_row_index(NUMBER_COLUMN_NAME, 1)
        .select(pl.col(NUMBER_COLUMN_NAME), pl.col(column.name).alias("source"))
        .collect(engine=QUERY_ENGINE)
    )

    for row in frame.to_dicts():
        if row["source"] is None:
            continue

        target = None
        try:
            raw = bytes.fromhex(row["source"])
            target = shapely.from_wkb(raw)
        except Exception:
            pass

        if target is None:
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
