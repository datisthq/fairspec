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

from ..enum import check_cell_enum


class TestCheckCellEnum:
    def test_returns_none_when_no_enum(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_enum(column, mapping)

        assert result is None

    def test_string_values_in_enum(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(
                type="string", enum=["pending", "approved", "rejected"]
            ),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["pending", "approved", "rejected", "pending"],
                "target": ["pending", "approved", "rejected", "pending"],
            }
        ).lazy()

        result = check_cell_enum(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_values_not_in_enum(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(
                type="string", enum=["pending", "approved", "rejected"]
            ),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["pending", "approved", "unknown", "cancelled", "rejected"],
                "target": ["pending", "approved", "unknown", "cancelled", "rejected"],
            }
        ).lazy()

        result = check_cell_enum(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/enum"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_null_values_not_flagged(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(
                type="string", enum=["pending", "approved", "rejected"]
            ),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["pending", None, "approved", None],
                "target": ["pending", None, "approved", None],
            }
        ).lazy()

        result = check_cell_enum(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_case_sensitivity(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(
                type="string", enum=["pending", "approved", "rejected"]
            ),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["Pending", "APPROVED", "rejected"],
                "target": ["Pending", "APPROVED", "rejected"],
            }
        ).lazy()

        result = check_cell_enum(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_integer_enum(self):
        column = IntegerColumn(
            name="priority",
            type="integer",
            property=IntegerColumnProperty(enum=[1, 2, 3]),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame({"source": [1, 2, 5], "target": [1, 2, 5]}).lazy()

        result = check_cell_enum(column, mapping)

        assert result is not None
        assert result.error_template.enum == ["1", "2", "3"]
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_number_enum(self):
        column = NumberColumn(
            name="rating",
            type="number",
            property=NumberColumnProperty(enum=[1.5, 2.5, 3.5]),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [1.5, 2.5, 4.5], "target": [1.5, 2.5, 4.5]}
        ).lazy()

        result = check_cell_enum(column, mapping)

        assert result is not None
        assert result.error_template.enum == ["1.5", "2.5", "3.5"]
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1
