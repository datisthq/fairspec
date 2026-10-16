from __future__ import annotations

import polars as pl
from fairspec_metadata import (
    IntegerColumn,
    IntegerColumnProperty,
)
from fairspec_metadata import (
    NumberColumn,
    NumberColumnProperty,
)
from fairspec_metadata import (
    StringColumn,
    StringColumnProperty,
)

from fairspec_table.models import CellMapping

from ..maximum import create_check_cell_maximum


class TestCheckCellMaximum:
    def test_returns_none_when_no_maximum(self):
        column = NumberColumn(
            name="price",
            type="number",
            property=NumberColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        check = create_check_cell_maximum()

        result = check(column, mapping)

        assert result is None

    def test_returns_none_for_column_without_maximum_field(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        check = create_check_cell_maximum()

        result = check(column, mapping)

        assert result is None

    def test_values_within_maximum(self):
        column = NumberColumn(
            name="price",
            type="number",
            property=NumberColumnProperty(maximum=50),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [10.5, 20.75, 30.0], "target": [10.5, 20.75, 30.0]}
        ).lazy()
        check = create_check_cell_maximum()

        result = check(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_values_exceeding_maximum(self):
        column = NumberColumn(
            name="temperature",
            type="number",
            property=NumberColumnProperty(maximum=40),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [20.5, 30.0, 40.0, 50.5], "target": [20.5, 30.0, 40.0, 50.5]}
        ).lazy()
        check = create_check_cell_maximum()

        result = check(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/maximum"
        assert result.error_template.maximum == "40.0"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_exclusive_maximum(self):
        column = NumberColumn(
            name="temperature",
            type="number",
            property=NumberColumnProperty(exclusiveMaximum=40),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [20.5, 30.0, 40.0, 50.5], "target": [20.5, 30.0, 40.0, 50.5]}
        ).lazy()
        check = create_check_cell_maximum(is_exclusive=True)

        result = check(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/exclusiveMaximum"
        assert result.error_template.maximum == "40.0"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_integer_maximum(self):
        column = IntegerColumn(
            name="year",
            type="integer",
            property=IntegerColumnProperty(maximum=2022),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [2020, 2021, 2023], "target": [2020, 2021, 2023]}
        ).lazy()
        check = create_check_cell_maximum()

        result = check(column, mapping)

        assert result is not None
        assert result.error_template.maximum == "2022"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_integer_exclusive_maximum(self):
        column = IntegerColumn(
            name="year",
            type="integer",
            property=IntegerColumnProperty(exclusiveMaximum=2022),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [2020, 2021, 2022, 2023], "target": [2020, 2021, 2022, 2023]}
        ).lazy()
        check = create_check_cell_maximum(is_exclusive=True)

        result = check(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2
