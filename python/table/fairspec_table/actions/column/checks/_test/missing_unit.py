from __future__ import annotations

import polars as pl
from fairspec_metadata import (
    NumberColumn,
    NumberColumnProperty,
)

from fairspec_table.models import CellMapping

from ..missing import check_cell_missing


class TestCheckCellMissing:
    def test_non_nullable_column_with_nulls(self):
        column = NumberColumn(
            name="id",
            type="number",
            property=NumberColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [1.0, None, 3.0], "target": [1.0, None, 3.0]}
        ).lazy()

        result = check_cell_missing(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/missing"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_nullable_column_returns_none(self):
        column = NumberColumn(
            name="id",
            type="number",
            nullable=True,
            property=NumberColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_missing(column, mapping)

        assert result is None

    def test_non_nullable_column_no_nulls(self):
        column = NumberColumn(
            name="id",
            type="number",
            property=NumberColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [1.0, 2.0, 3.0], "target": [1.0, 2.0, 3.0]}
        ).lazy()

        result = check_cell_missing(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_error_template_fields(self):
        column = NumberColumn(
            name="id",
            type="number",
            property=NumberColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_missing(column, mapping)

        assert result is not None
        assert result.error_template.columnName == "id"
        assert result.error_template.rowNumber == 0
        assert result.error_template.cell == ""
