from __future__ import annotations

import datetime

import polars as pl
import pytest

from fairspec_metadata import DateColumn, DateColumnProperty

from ..date import parse_date_column, stringify_date_column


class TestParseDateColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("2019-01-01", datetime.date(2019, 1, 1)),
            ("10th Jan 1969", None),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_default(self, cell: str, expected: datetime.date | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DateColumn(
            name="name",
            type="date",
            property=DateColumnProperty(),
        )

        result = table.select(parse_date_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("21/11/2006", datetime.date(2006, 11, 21)),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_temporal_format_dmy(self, cell: str, expected: datetime.date | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DateColumn(
            name="name",
            type="date",
            property=DateColumnProperty(temporalFormat="%d/%m/%Y"),
        )

        result = table.select(parse_date_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("2006/11/21", datetime.date(2006, 11, 21)),
        ],
    )
    def test_temporal_format_ymd(self, cell: str, expected: datetime.date):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DateColumn(
            name="name",
            type="date",
            property=DateColumnProperty(temporalFormat="%Y/%m/%d"),
        )

        result = table.select(parse_date_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("21/11/06", None),
        ],
    )
    def test_invalid_temporal_format(self, cell: str, expected: datetime.date | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DateColumn(
            name="name",
            type="date",
            property=DateColumnProperty(temporalFormat="invalid"),
        )

        result = table.select(parse_date_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyDateColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (datetime.date(2019, 1, 1), "2019-01-01"),
            (datetime.date(2006, 11, 21), "2006-11-21"),
        ],
    )
    def test_default(self, value: datetime.date, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Date)}).lazy()
        column = DateColumn(
            name="name",
            type="date",
            property=DateColumnProperty(),
        )

        result = table.select(stringify_date_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (datetime.date(2006, 11, 21), "21/11/2006"),
        ],
    )
    def test_temporal_format_dmy(self, value: datetime.date, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Date)}).lazy()
        column = DateColumn(
            name="name",
            type="date",
            property=DateColumnProperty(temporalFormat="%d/%m/%Y"),
        )

        result = table.select(stringify_date_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (datetime.date(2006, 11, 21), "2006/11/21"),
        ],
    )
    def test_temporal_format_ymd(self, value: datetime.date, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Date)}).lazy()
        column = DateColumn(
            name="name",
            type="date",
            property=DateColumnProperty(temporalFormat="%Y/%m/%d"),
        )

        result = table.select(stringify_date_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
