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

from ..pattern import check_cell_pattern


class TestCheckCellPattern:
    def test_returns_none_when_no_pattern(self):
        column = StringColumn(
            name="email",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_pattern(column, mapping)

        assert result is None

    def test_returns_none_for_column_without_pattern_field(self):
        column = IntegerColumn(
            name="id",
            type="integer",
            property=IntegerColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_pattern(column, mapping)

        assert result is None

    def test_values_matching_pattern(self):
        email_pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        column = StringColumn(
            name="email",
            type="string",
            property=StringColumnProperty(pattern=email_pattern),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["john@example.com", "alice@domain.org", "test@test.io"],
                "target": ["john@example.com", "alice@domain.org", "test@test.io"],
            }
        ).lazy()

        result = check_cell_pattern(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_values_not_matching_pattern(self):
        email_pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
        column = StringColumn(
            name="email",
            type="string",
            property=StringColumnProperty(pattern=email_pattern),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": [
                    "john@example.com",
                    "alice@domain",
                    "test.io",
                    "valid@email.com",
                ],
                "target": [
                    "john@example.com",
                    "alice@domain",
                    "test.io",
                    "valid@email.com",
                ],
            }
        ).lazy()

        result = check_cell_pattern(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/pattern"
        assert result.error_template.pattern == email_pattern
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2
