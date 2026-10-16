from __future__ import annotations

import datetime

import polars as pl
import pytest

from fairspec_metadata import DateTimeColumn, DateTimeColumnProperty

from ..date_time import parse_date_time_column, stringify_date_time_column


class TestParseDateTimeColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            (
                "2014-01-01T06:00:00",
                datetime.datetime(2014, 1, 1, 6, 0, 0),
            ),
            ("Mon 1st Jan 2014 9 am", None),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_default(self, cell: str, expected: datetime.datetime | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DateTimeColumn(
            name="name",
            type="date-time",
            property=DateTimeColumnProperty(),
        )

        result = table.select(parse_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            (
                "21/11/2006 16:30",
                datetime.datetime(2006, 11, 21, 16, 30),
            ),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_temporal_format(self, cell: str, expected: datetime.datetime | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DateTimeColumn(
            name="name",
            type="date-time",
            property=DateTimeColumnProperty(
                format="date-time",
                temporalFormat="%d/%m/%Y %H:%M",
            ),
        )

        result = table.select(parse_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("21/11/06 16:30", None),
        ],
    )
    def test_invalid_temporal_format(self, cell: str, expected: datetime.datetime | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DateTimeColumn(
            name="name",
            type="date-time",
            property=DateTimeColumnProperty(temporalFormat="invalid"),
        )

        result = table.select(parse_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyDateTimeColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (
                datetime.datetime(2014, 1, 1, 6, 0, 0),
                "2014-01-01T06:00:00",
            ),
            (
                datetime.datetime(2006, 11, 21, 16, 30, 0),
                "2006-11-21T16:30:00",
            ),
        ],
    )
    def test_default(self, value: datetime.datetime, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Datetime)}).lazy()
        column = DateTimeColumn(
            name="name",
            type="date-time",
            property=DateTimeColumnProperty(),
        )

        result = table.select(stringify_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (
                datetime.datetime(2006, 11, 21, 16, 30, 0),
                "21/11/2006 16:30",
            ),
        ],
    )
    def test_temporal_format_dmy_hm(self, value: datetime.datetime, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Datetime)}).lazy()
        column = DateTimeColumn(
            name="name",
            type="date-time",
            property=DateTimeColumnProperty(
                format="date-time",
                temporalFormat="%d/%m/%Y %H:%M",
            ),
        )

        result = table.select(stringify_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (
                datetime.datetime(2014, 1, 1, 6, 0, 0),
                "2014/01/01T06:00:00",
            ),
        ],
    )
    def test_temporal_format_ymd_hms(self, value: datetime.datetime, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Datetime)}).lazy()
        column = DateTimeColumn(
            name="name",
            type="date-time",
            property=DateTimeColumnProperty(
                format="date-time",
                temporalFormat="%Y/%m/%dT%H:%M:%S",
            ),
        )

        result = table.select(stringify_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
