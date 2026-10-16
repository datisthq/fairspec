from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import CategoricalColumn, IntegerColumn
from fairspec_metadata import (
    IntegerCategoricalColumnProperty,
    IntegerCategoryItem,
    StringCategoricalColumnProperty,
)
from fairspec_metadata import IntegerColumnProperty

from fairspec_table.models import ColumnMapping, PolarsColumn

from ..narrow import narrow_column


class TestNarrowToInteger:
    def test_narrow_float_to_integer(self):
        table = pl.DataFrame({"id": [1.0, 2.0, 3.0]}).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="id", type=pl.Float64),
            target=IntegerColumn(
                name="id",
                type="integer",
                property=IntegerColumnProperty(),
            ),
        )

        result = table.select(narrow_column(mapping, pl.col("id")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [
            {"id": 1},
            {"id": 2},
            {"id": 3},
        ]

    def test_non_integer_float_becomes_null(self):
        table = pl.DataFrame({"id": [1.0, 2.0, 3.5]}).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="id", type=pl.Float64),
            target=IntegerColumn(
                name="id",
                type="integer",
                property=IntegerColumnProperty(),
            ),
        )

        result = table.select(narrow_column(mapping, pl.col("id")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [
            {"id": 1},
            {"id": 2},
            {"id": None},
        ]


class TestNarrowToCategorical:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("red", "red"),
            ("green", "green"),
            ("yellow", None),
        ],
    )
    def test_string_categorical(self, cell: str, expected: str | None):
        table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
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

        result = table.select(narrow_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            (1, "Low"),
            (2, "High"),
            (3, None),
        ],
    )
    def test_integer_categorical(self, cell: int, expected: str | None):
        table = pl.DataFrame([pl.Series("name", [cell], pl.Int64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.Int64),
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

        result = table.select(narrow_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
