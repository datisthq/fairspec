from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import IntegerColumn, NumberColumn, StringColumn
from fairspec_metadata import IntegerColumnProperty
from fairspec_metadata import NumberColumnProperty
from fairspec_metadata import StringColumnProperty

from fairspec_table.models import ColumnMapping, PolarsColumn

from ..desubstitute import desubstitute_column


class TestDesubstituteColumnString:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, None),
            ("hello", "hello"),
            ("value", "value"),
        ],
    )
    def test_no_missing_values(self, value: str | None, expected: str | None):
        table = pl.DataFrame([pl.Series("name", [value], pl.String)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(),
            ),
        )

        result = table.select(desubstitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, None),
            ("hello", "hello"),
            ("value", "value"),
        ],
    )
    def test_empty_missing_values(self, value: str | None, expected: str | None):
        table = pl.DataFrame([pl.Series("name", [value], pl.String)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(missingValues=[]),
            ),
        )

        result = table.select(desubstitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, "-"),
            ("hello", "hello"),
            ("value", "value"),
        ],
    )
    def test_missing_value_dash(self, value: str | None, expected: str):
        table = pl.DataFrame([pl.Series("name", [value], pl.String)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(missingValues=["-"]),
            ),
        )

        result = table.select(desubstitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, "x"),
            ("hello", "hello"),
            ("value", "value"),
        ],
    )
    def test_missing_value_x(self, value: str | None, expected: str):
        table = pl.DataFrame([pl.Series("name", [value], pl.String)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(missingValues=["x"]),
            ),
        )

        result = table.select(desubstitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, "n/a"),
            ("value", "value"),
            ("test", "test"),
        ],
    )
    def test_missing_value_na(self, value: str | None, expected: str):
        table = pl.DataFrame([pl.Series("name", [value], pl.String)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(missingValues=["n/a"]),
            ),
        )

        result = table.select(desubstitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, "-"),
            ("value", "value"),
            ("test", "test"),
        ],
    )
    def test_multiple_missing_values_uses_first(self, value: str | None, expected: str):
        table = pl.DataFrame([pl.Series("name", [value], pl.String)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="name", type=pl.String),
            target=StringColumn(
                name="name",
                type="string",
                property=StringColumnProperty(
                    type="string", missingValues=["-", "n/a", "null"]
                ),
            ),
        )

        result = table.select(desubstitute_column(mapping, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestDesubstituteColumnInteger:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, -1),
            (0, 0),
            (1, 1),
            (42, 42),
        ],
    )
    def test_missing_value_neg1(self, value: int | None, expected: int):
        table = pl.DataFrame([pl.Series("value", [value], pl.Int64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Int64),
            target=IntegerColumn(
                name="value",
                type="integer",
                property=IntegerColumnProperty(missingValues=[-1]),
            ),
        )

        result = table.select(
            desubstitute_column(
                mapping,
                pl.col("value"),
                nativeTypes=["integer"],
            )
        )

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"value": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, -999),
            (0, 0),
            (1, 1),
            (100, 100),
        ],
    )
    def test_missing_value_neg999(self, value: int | None, expected: int):
        table = pl.DataFrame([pl.Series("value", [value], pl.Int64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Int64),
            target=IntegerColumn(
                name="value",
                type="integer",
                property=IntegerColumnProperty(missingValues=[-999]),
            ),
        )

        result = table.select(
            desubstitute_column(
                mapping,
                pl.col("value"),
                nativeTypes=["integer"],
            )
        )

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"value": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, -1),
            (0, 0),
            (42, 42),
        ],
    )
    def test_multiple_missing_values_uses_first(self, value: int | None, expected: int):
        table = pl.DataFrame([pl.Series("value", [value], pl.Int64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Int64),
            target=IntegerColumn(
                name="value",
                type="integer",
                property=IntegerColumnProperty(
                    type="integer", missingValues=[-1, -999, -9999]
                ),
            ),
        )

        result = table.select(
            desubstitute_column(
                mapping,
                pl.col("value"),
                nativeTypes=["integer"],
            )
        )

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"value": expected}]


class TestDesubstituteColumnNumber:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, -1.0),
            (0.0, 0.0),
            (1.0, 1.0),
            (42.0, 42.0),
        ],
    )
    def test_missing_value_float64(self, value: float | None, expected: float):
        table = pl.DataFrame([pl.Series("value", [value], pl.Float64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Float64),
            target=NumberColumn(
                name="value",
                type="number",
                property=NumberColumnProperty(missingValues=[-1]),
            ),
        )

        result = table.select(
            desubstitute_column(
                mapping,
                pl.col("value"),
                nativeTypes=["number"],
            )
        )

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"value": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, -999.0),
            (0.0, 0.0),
            (1.0, 1.0),
            (100.0, 100.0),
        ],
    )
    def test_missing_value_neg999_float64(self, value: float | None, expected: float):
        table = pl.DataFrame([pl.Series("value", [value], pl.Float64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Float64),
            target=NumberColumn(
                name="value",
                type="number",
                property=NumberColumnProperty(missingValues=[-999]),
            ),
        )

        result = table.select(
            desubstitute_column(
                mapping,
                pl.col("value"),
                nativeTypes=["number"],
            )
        )

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"value": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (None, -1.0),
            (0.0, 0.0),
            (42.0, 42.0),
        ],
    )
    def test_multiple_missing_values_uses_first_float64(
        self, value: float | None, expected: float
    ):
        table = pl.DataFrame([pl.Series("value", [value], pl.Float64)]).lazy()
        mapping = ColumnMapping(
            source=PolarsColumn(name="value", type=pl.Float64),
            target=NumberColumn(
                name="value",
                type="number",
                property=NumberColumnProperty(
                    type="number", missingValues=[-1, -999, -9999]
                ),
            ),
        )

        result = table.select(
            desubstitute_column(
                mapping,
                pl.col("value"),
                nativeTypes=["number"],
            )
        )

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"value": expected}]
