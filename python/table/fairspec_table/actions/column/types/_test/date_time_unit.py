from __future__ import annotations

import datetime

import polars as pl
import pytest

from fairspec_metadata import DateTimeColumn, DateTimeColumnProperty

from ..date_time import parse_date_time_column, stringify_date_time_column

UTC = datetime.timezone.utc


class TestParseDateTimeColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("2014-01-01T06:00:00Z", datetime.datetime(2014, 1, 1, 6, tzinfo=UTC)),
            ("2014-01-01T06:00:00+00:00", datetime.datetime(2014, 1, 1, 6, tzinfo=UTC)),
            ("2014-01-01T06:00:00-05:00", datetime.datetime(2014, 1, 1, 11, tzinfo=UTC)),
            (
                "2014-01-01T06:00:00.25+02:00",
                datetime.datetime(2014, 1, 1, 4, 0, 0, 250000, tzinfo=UTC),
            ),
            ("2014-01-01T06:00:00", None),
            ("2014-01-01T06:00:00+02", None),
            ("2014-01-01T06:00:00+0200", None),
            ("2014-01-01T06:00:00Zx", None),
            ("x2014-01-01T06:00:00Z", None),
            ("2014-01-01", None),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_default(self, cell: str, expected: datetime.datetime | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DateTimeColumn(
            name="name", type="date-time", property=DateTimeColumnProperty()
        )

        result = table.select(parse_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            (
                "21/11/2006 16:30+0200",
                datetime.datetime(2006, 11, 21, 14, 30, tzinfo=UTC),
            ),
            (
                "21/11/2006 16:30+02:00",
                datetime.datetime(2006, 11, 21, 14, 30, tzinfo=UTC),
            ),
            ("21/11/2006 16:30", None),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_temporal_format(self, cell: str, expected: datetime.datetime | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DateTimeColumn(
            name="name",
            type="date-time",
            property=DateTimeColumnProperty(temporalFormat="%d/%m/%Y %H:%M%z"),
        )

        result = table.select(parse_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyDateTimeColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (datetime.datetime(2014, 1, 1, 6), "2014-01-01T06:00:00Z"),
            (datetime.datetime(2014, 1, 1, 6, 0, 0, 125000), "2014-01-01T06:00:00.125Z"),
        ],
    )
    def test_default(self, value: datetime.datetime, expected: str):
        table = pl.DataFrame(
            {"name": pl.Series([value], dtype=pl.Datetime("us", "UTC"))}
        ).lazy()
        column = DateTimeColumn(
            name="name", type="date-time", property=DateTimeColumnProperty()
        )

        result = table.select(stringify_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    def test_temporal_format(self):
        value = datetime.datetime(2006, 11, 21, 16, 30)
        table = pl.DataFrame(
            {"name": pl.Series([value], dtype=pl.Datetime("us", "UTC"))}
        ).lazy()
        column = DateTimeColumn(
            name="name",
            type="date-time",
            property=DateTimeColumnProperty(temporalFormat="%d/%m/%Y %H:%M%:z"),
        )

        result = table.select(stringify_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": "21/11/2006 16:30+00:00"}]
