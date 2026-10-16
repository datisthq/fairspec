from __future__ import annotations

import polars as pl
from fairspec_metadata import (
    BooleanColumn,
    BooleanColumnProperty,
)
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

from ..const import check_cell_const


class TestCheckCellConst:
    def test_returns_none_when_no_const(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_const(column, mapping)

        assert result is None

    def test_string_const_match(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(const="active"),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["active", "active", "active"],
                "target": ["active", "active", "active"],
            }
        ).lazy()

        result = check_cell_const(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/const"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_string_const_mismatch(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(const="active"),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["active", "inactive", "active", "pending"],
                "target": ["active", "inactive", "active", "pending"],
            }
        ).lazy()

        result = check_cell_const(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_null_values_not_flagged(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(const="active"),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["active", None, "active", None],
                "target": ["active", None, "active", None],
            }
        ).lazy()

        result = check_cell_const(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_case_sensitivity(self):
        column = StringColumn(
            name="status",
            type="string",
            property=StringColumnProperty(const="active"),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["Active", "ACTIVE", "active"],
                "target": ["Active", "ACTIVE", "active"],
            }
        ).lazy()

        result = check_cell_const(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_integer_const(self):
        column = IntegerColumn(
            name="priority",
            type="integer",
            property=IntegerColumnProperty(const=1),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame({"source": [1, 1, 2], "target": [1, 1, 2]}).lazy()

        result = check_cell_const(column, mapping)

        assert result is not None
        assert result.error_template.model_dump()["const"] == "1"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_number_const(self):
        column = NumberColumn(
            name="rate",
            type="number",
            property=NumberColumnProperty(const=1.5),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [1.5, 1.5, 2.5], "target": [1.5, 1.5, 2.5]}
        ).lazy()

        result = check_cell_const(column, mapping)

        assert result is not None
        assert result.error_template.model_dump()["const"] == "1.5"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_boolean_const(self):
        column = BooleanColumn(
            name="enabled",
            type="boolean",
            property=BooleanColumnProperty(const=True),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": [True, True, False], "target": [True, True, False]}
        ).lazy()

        result = check_cell_const(column, mapping)

        assert result is not None
        assert result.error_template.model_dump()["const"] == "True"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1
