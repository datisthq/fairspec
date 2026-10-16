from __future__ import annotations

import polars as pl
from fairspec_metadata import ListColumn, ListColumnProperty
from fairspec_metadata import (
    StringColumn,
    StringColumnProperty,
)

from fairspec_table.models import CellMapping

from ..max_items import check_cell_max_items


class TestCheckCellMaxItems:
    def test_returns_none_for_non_list_column(self):
        column = StringColumn(
            name="tags",
            type="string",
            property=StringColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_max_items(column, mapping)

        assert result is None

    def test_returns_none_when_no_max_items(self):
        column = ListColumn(
            name="tags",
            type="list",
            property=ListColumnProperty(),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))

        result = check_cell_max_items(column, mapping)

        assert result is None

    def test_values_within_max_items(self):
        column = ListColumn(
            name="tags",
            type="list",
            property=ListColumnProperty(maxItems=3),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["a,b", "x,y,z", "1"],
                "target": [["a", "b"], ["x", "y", "z"], ["1"]],
            }
        ).lazy()

        result = check_cell_max_items(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 0

    def test_values_exceeding_max_items(self):
        column = ListColumn(
            name="tags",
            type="list",
            property=ListColumnProperty(maxItems=3),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["a,b", "x,y,z,w", "1,2,3,4,5", "p,q"],
                "target": [
                    ["a", "b"],
                    ["x", "y", "z", "w"],
                    ["1", "2", "3", "4", "5"],
                    ["p", "q"],
                ],
            }
        ).lazy()

        result = check_cell_max_items(column, mapping)

        assert result is not None
        assert result.error_template.type == "cell/maxItems"
        assert result.error_template.maxItems == 3
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 2

    def test_null_values_not_flagged(self):
        column = ListColumn(
            name="tags",
            type="list",
            property=ListColumnProperty(maxItems=2),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["a,b", None, "x,y,z,w,v"],
                "target": [["a", "b"], None, ["x", "y", "z", "w", "v"]],
            }
        ).lazy()

        result = check_cell_max_items(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1

    def test_max_items_of_one(self):
        column = ListColumn(
            name="tags",
            type="list",
            property=ListColumnProperty(maxItems=1),
        )
        mapping = CellMapping(source=pl.col("source"), target=pl.col("target"))
        table = pl.DataFrame(
            {
                "source": ["a", "b,c", "d"],
                "target": [["a"], ["b", "c"], ["d"]],
            }
        ).lazy()

        result = check_cell_max_items(column, mapping)

        assert result is not None
        errors: pl.DataFrame = table.filter(result.is_error_expr).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert len(errors) == 1
