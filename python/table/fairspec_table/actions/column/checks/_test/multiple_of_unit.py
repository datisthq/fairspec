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

from ..multiple_of import check_cell_multiple_of


class TestCheckCellMultipleOf:
    def test_returns_none_when_no_multiple_of(self):
        column = IntegerColumn(
            name="quantity",
            type="integer",
            property=IntegerColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_multiple_of(column, mapping)

        assert result is None

    def test_returns_none_for_column_without_multiple_of_field(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_multiple_of(column, mapping)

        assert result is None

    def test_valid_integer_multiples(self):
        column = IntegerColumn(
            name="quantity",
            type="integer",
            property=IntegerColumnProperty(multipleOf=10),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [10, 20, 30, 40], "target": [10, 20, 30, 40]}
        ).lazy()

        result = check_cell_multiple_of(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_invalid_integer_values(self):
        column = IntegerColumn(
            name="quantity",
            type="integer",
            property=IntegerColumnProperty(multipleOf=10),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame({"source": [10, 15, 20], "target": [10, 15, 20]}).lazy()

        result = check_cell_multiple_of(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/multipleOf"
        assert result.error_template.multipleOf == 10
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_valid_number_multiples(self):
        column = NumberColumn(
            name="price",
            type="number",
            property=NumberColumnProperty(multipleOf=2.5),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [2.5, 5.0, 7.5], "target": [2.5, 5.0, 7.5]}
        ).lazy()

        result = check_cell_multiple_of(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_invalid_number_values(self):
        column = NumberColumn(
            name="price",
            type="number",
            property=NumberColumnProperty(multipleOf=2.5),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [2.5, 3.7, 5.0], "target": [2.5, 3.7, 5.0]}
        ).lazy()

        result = check_cell_multiple_of(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_multiple_of_one(self):
        column = IntegerColumn(
            name="count",
            type="integer",
            property=IntegerColumnProperty(multipleOf=1),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame({"source": [1, 2, 3, 4], "target": [1, 2, 3, 4]}).lazy()

        result = check_cell_multiple_of(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0
