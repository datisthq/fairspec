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

from ..max_length import check_cell_max_length


class TestCheckCellMaxLength:
    def test_returns_none_when_no_max_length(self):
        column = StringColumn(
            name="code",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_max_length(column, mapping)

        assert result is None

    def test_returns_none_for_column_without_max_length_field(self):
        column = IntegerColumn(
            name="id",
            type="integer",
            property=IntegerColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_max_length(column, mapping)

        assert result is None

    def test_values_within_max_length(self):
        column = StringColumn(
            name="code",
            type="string",
            property=StringColumnProperty(maxLength=4),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": ["A123", "B456", "C789"], "target": ["A123", "B456", "C789"]}
        ).lazy()

        result = check_cell_max_length(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_values_exceeding_max_length(self):
        column = StringColumn(
            name="username",
            type="string",
            property=StringColumnProperty(maxLength=8),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["bob", "alice", "christopher", "david"],
                "target": ["bob", "alice", "christopher", "david"],
            }
        ).lazy()

        result = check_cell_max_length(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/maxLength"
        assert result.error_template.maxLength == 8
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1
        assert errors["source"][0] == "christopher"
