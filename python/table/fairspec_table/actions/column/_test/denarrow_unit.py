from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import CategoricalColumn
from fairspec_metadata import (
    IntegerCategoricalColumnProperty,
    IntegerCategoryItem,
    StringCategoricalColumnProperty,
)

from fairspec_table.models import ColumnMapping, PolarsColumn

from ..denarrow import denarrow_column


class TestDenarrowCategorical:
    @pytest.mark.parametrize(
        "value, expected",
        [
            ("red", "red"),
            ("green", "green"),
        ],
    )
    def test_string_categorical(self, value: str, expected: str):
        table = pl.DataFrame([pl.Series("name", [value], pl.Categorical)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.Categorical),
            target=CategoricalColumn(
                name="name",
                type="categorical",
                property=StringCategoricalColumnProperty(
                    type="string",
                    format="categorical",
                    categories=["red", "green", "blue"],
                ),
            ),
        )

        result = table.select(denarrow_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            ("Low", 1),
            ("High", 2),
        ],
    )
    def test_integer_categorical(self, value: str, expected: int):
        table = pl.DataFrame([pl.Series("name", [value], pl.Categorical)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.Categorical),
            target=CategoricalColumn(
                name="name",
                type="categorical",
                property=IntegerCategoricalColumnProperty(
                    type="integer",
                    format="categorical",
                    categories=[
                        IntegerCategoryItem(value=1, label="Low"),
                        IntegerCategoryItem(value=2, label="High"),
                    ],
                ),
            ),
        )

        result = table.select(denarrow_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
