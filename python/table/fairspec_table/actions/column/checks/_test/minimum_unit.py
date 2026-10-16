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

from ..minimum import create_check_cell_minimum


class TestCheckCellMinimum:
    def test_returns_none_when_no_minimum(self):
        column = NumberColumn(
            name="price",
            type="number",
            property=NumberColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        check = create_check_cell_minimum()

        result = check(column, mapping)

        assert result is None

    def test_returns_none_for_column_without_minimum_field(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        check = create_check_cell_minimum()

        result = check(column, mapping)

        assert result is None

    def test_values_above_minimum(self):
        column = NumberColumn(
            name="price",
            type="number",
            property=NumberColumnProperty(minimum=5),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [10.5, 20.75, 30.0], "target": [10.5, 20.75, 30.0]}
        ).lazy()
        check = create_check_cell_minimum()

        result = check(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_values_below_minimum(self):
        column = NumberColumn(
            name="temperature",
            type="number",
            property=NumberColumnProperty(minimum=10),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [20.5, 30.0, 40.0, 3.5], "target": [20.5, 30.0, 40.0, 3.5]}
        ).lazy()
        check = create_check_cell_minimum()

        result = check(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/minimum"
        assert result.error_template.minimum == "10.0"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_exclusive_minimum(self):
        column = NumberColumn(
            name="temperature",
            type="number",
            property=NumberColumnProperty(exclusiveMinimum=10),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [20.5, 30.0, 10.0, 5.5], "target": [20.5, 30.0, 10.0, 5.5]}
        ).lazy()
        check = create_check_cell_minimum(is_exclusive=True)

        result = check(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/exclusiveMinimum"
        assert result.error_template.minimum == "10.0"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_integer_minimum(self):
        column = IntegerColumn(
            name="year",
            type="integer",
            property=IntegerColumnProperty(minimum=2019),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [2020, 2021, 2018], "target": [2020, 2021, 2018]}
        ).lazy()
        check = create_check_cell_minimum()

        result = check(column, mapping)

        assert result is not None
        assert result.error_template.minimum == "2019"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_integer_exclusive_minimum(self):
        column = IntegerColumn(
            name="year",
            type="integer",
            property=IntegerColumnProperty(exclusiveMinimum=2019),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [2020, 2021, 2019, 2018], "target": [2020, 2021, 2019, 2018]}
        ).lazy()
        check = create_check_cell_minimum(is_exclusive=True)

        result = check(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2
