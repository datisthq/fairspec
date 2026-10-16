from __future__ import annotations

import polars as pl
from fairspec_metadata import (
    IntegerColumn,
    IntegerColumnProperty,
)
from fairspec_metadata import (
    StringColumn,
    StringColumnProperty,
)

from fairspec_table.models import CellMapping

from ..min_length import check_cell_min_length


class TestCheckCellMinLength:
    def test_returns_none_when_no_min_length(self):
        column = StringColumn(
            name="code",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_min_length(column, mapping)

        assert result is None

    def test_returns_none_for_column_without_min_length_field(self):
        column = IntegerColumn(
            name="id",
            type="integer",
            property=IntegerColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_min_length(column, mapping)

        assert result is None

    def test_values_meeting_min_length(self):
        column = StringColumn(
            name="code",
            type="string",
            property=StringColumnProperty(minLength=3),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": ["A123", "B456", "C789"], "target": ["A123", "B456", "C789"]}
        ).lazy()

        result = check_cell_min_length(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_values_below_min_length(self):
        column = StringColumn(
            name="username",
            type="string",
            property=StringColumnProperty(minLength=3),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["bob", "a", "christopher", "ab"],
                "target": ["bob", "a", "christopher", "ab"],
            }
        ).lazy()

        result = check_cell_min_length(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/minLength"
        assert result.error_template.minLength == 3
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_null_values_not_flagged(self):
        column = StringColumn(
            name="code",
            type="string",
            property=StringColumnProperty(minLength=3),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": ["ABCD", "A", None], "target": ["ABCD", "A", None]}
        ).lazy()

        result = check_cell_min_length(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1
