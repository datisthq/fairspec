from __future__ import annotations

import polars as pl
from fairspec_metadata import ListColumn, ListColumnProperty
from fairspec_metadata import (
    StringColumn,
    StringColumnProperty,
)

from fairspec_table.models import CellMapping

from ..min_items import check_cell_min_items


class TestCheckCellMinItems:
    def test_returns_none_for_non_list_column(self):
        column = StringColumn(
            name="tags",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_min_items(column, mapping)

        assert result is None

    def test_returns_none_when_no_min_items(self):
        column = ListColumn(
            name="tags",
            type="list",
            property=ListColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_min_items(column, mapping)

        assert result is None

    def test_values_meeting_min_items(self):
        column = ListColumn(
            name="tags",
            type="list",
            property=ListColumnProperty(minItems=3),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["a,b,c", "x,y,z", "1,2,3"],
                "target": [
                    ["a", "b", "c"],
                    ["x", "y", "z"],
                    ["1", "2", "3"],
                ],
            }
        ).lazy()

        result = check_cell_min_items(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_values_below_min_items(self):
        column = ListColumn(
            name="tags",
            type="list",
            property=ListColumnProperty(minItems=3),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["a,b,c", "x", "1,2", "p,q,r,s"],
                "target": [
                    ["a", "b", "c"],
                    ["x"],
                    ["1", "2"],
                    ["p", "q", "r", "s"],
                ],
            }
        ).lazy()

        result = check_cell_min_items(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/minItems"
        assert result.error_template.minItems == 3
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_null_values_not_flagged(self):
        column = ListColumn(
            name="tags",
            type="list",
            property=ListColumnProperty(minItems=3),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["a,b,c", None, "x,y,z"],
                "target": [["a", "b", "c"], None, ["x", "y", "z"]],
            }
        ).lazy()

        result = check_cell_min_items(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0
