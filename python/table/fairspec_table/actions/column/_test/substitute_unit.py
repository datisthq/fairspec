from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import IntegerColumn, StringColumn
from fairspec_metadata import IntegerColumnProperty
from fairspec_metadata import StringColumnProperty

from fairspec_table.models import ColumnMapping, PolarsColumn

from ..substitute import substitute_column


class TestSubstituteColumnString:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("x", None),
            ("-", "-"),
            ("", ""),
            ("value", "value"),
        ],
    )
    def test_missing_value_x(self, cell: str, expected: str | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(missingValues=["x"]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("-", None),
            ("", ""),
            ("x", "x"),
            ("value", "value"),
        ],
    )
    def test_missing_value_dash(self, cell: str, expected: str | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(missingValues=["-"]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("", None),
            ("-", "-"),
            ("x", "x"),
            ("value", "value"),
        ],
    )
    def test_missing_value_empty(self, cell: str, expected: str | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(missingValues=[""]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("n/a", None),
            ("-", "-"),
            ("", ""),
            ("value", "value"),
        ],
    )
    def test_missing_value_na(self, cell: str, expected: str | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(missingValues=["n/a"]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("-", None),
            ("x", None),
            ("", ""),
            ("value", "value"),
        ],
    )
    def test_multiple_missing_values(self, cell: str, expected: str | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(missingValues=["-", "x"]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestSubstituteColumnInteger:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (-1, None),
            (0, 0),
            (1, 1),
            (42, 42),
        ],
    )
    def test_missing_value_int64(self, value: int, expected: int | None):
        table = pl.DataFrame([pl.Series("value", [value], pl.Int64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Int64),
            target=IntegerColumn(
                name="value",
                type="integer",
                property=IntegerColumnProperty(missingValues=[-1]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("value")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"value": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (-999, None),
            (0, 0),
            (1, 1),
            (100, 100),
        ],
    )
    def test_missing_value_neg999_int64(self, value: int, expected: int | None):
        table = pl.DataFrame([pl.Series("value", [value], pl.Int64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Int64),
            target=IntegerColumn(
                name="value",
                type="integer",
                property=IntegerColumnProperty(missingValues=[-999]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("value")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"value": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (-1, None),
            (-99, None),
            (0, 0),
            (42, 42),
        ],
    )
    def test_multiple_missing_values_int64(self, value: int, expected: int | None):
        table = pl.DataFrame([pl.Series("value", [value], pl.Int64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Int64),
            target=IntegerColumn(
                name="value",
                type="integer",
                property=IntegerColumnProperty(missingValues=[-1, -99]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("value")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"value": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (-1, None),
            (0, 0),
            (1, 1),
            (42, 42),
        ],
    )
    def test_missing_value_float64(self, value: int, expected: int | None):
        table = pl.DataFrame([pl.Series("value", [float(value)], pl.Float64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Float64),
            target=IntegerColumn(
                name="value",
                type="integer",
                property=IntegerColumnProperty(missingValues=[-1]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("value")))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        row = frame.to_dicts()[0]

        if expected is None:
            assert row["value"] is None
        else:
            assert row["value"] == float(expected)

    @pytest.mark.parametrize(
        "value, expected",
        [
            (-1, None),
            (-99, None),
            (0, 0),
            (42, 42),
        ],
    )
    def test_multiple_missing_values_float64(self, value: int, expected: int | None):
        table = pl.DataFrame([pl.Series("value", [float(value)], pl.Float64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Float64),
            target=IntegerColumn(
                name="value",
                type="integer",
                property=IntegerColumnProperty(missingValues=[-1, -99]),
            ),
        )

        result = table.select(substitute_column(mapping, pl.col("value")))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        row = frame.to_dicts()[0]

        if expected is None:
            assert row["value"] is None
        else:
            assert row["value"] == float(expected)
