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

from ..type import check_cell_type


class TestCheckCellType:
    def test_no_errors_for_valid_values(self):
        column = IntegerColumn(
            name="id",
            type="integer",
            property=IntegerColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": ["1", "2", "3", "4"], "target": [1, 2, 3, 4]}
        ).lazy()

        result = check_cell_type(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_errors_for_invalid_integer_values(self):
        column = IntegerColumn(
            name="id",
            type="integer",
            property=IntegerColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {"source": ["1", "bad", "3", "4x"], "target": [1, None, 3, None]}
        ).lazy()

        result = check_cell_type(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/type"
        assert result.error_template.columnType == "integer"
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_errors_for_invalid_number_values(self):
        column = NumberColumn(
            name="price",
            type="number",
            property=NumberColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["10.5", "twenty", "30.75", "$40"],
                "target": [10.5, None, 30.75, None],
            }
        ).lazy()

        result = check_cell_type(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_errors_for_invalid_boolean_values(self):
        column = BooleanColumn(
            name="active",
            type="boolean",
            property=BooleanColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["true", "yes", "false", "0", "1"],
                "target": [True, None, False, False, True],
            }
        ).lazy()

        result = check_cell_type(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_null_source_not_flagged(self):
        column = IntegerColumn(
            name="id",
            type="integer",
            property=IntegerColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame({"source": ["1", None, "3"], "target": [1, None, 3]}).lazy()

        result = check_cell_type(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_error_template_fields(self):
        column = StringColumn(
            name="name",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_type(column, mapping)

        assert result.error_template.columnName == "name"
        assert result.error_template.columnType == "string"
        assert result.error_template.rowNumber == 0
        assert result.error_template.cell == ""
