from __future__ import annotations

import datetime

import polars as pl
import pytest

from fairspec_metadata import LocalDateTimeColumn, LocalDateTimeColumnProperty

from ..local_date_time import (
    parse_local_date_time_column,
    stringify_local_date_time_column,
)


class TestParseLocalDateTimeColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            (
                "2014-01-01T06:00:00",
                datetime.datetime(2014, 1, 1, 6, 0, 0),
            ),
            ("2014-01-01T06:00:00Z", None),
            ("2014-01-01t06:00:00", datetime.datetime(2014, 1, 1, 6)),
            ("2016-12-31T23:59:60", datetime.datetime(2016, 12, 31, 23, 59, 59)),
            ("2016-12-31T12:30:60", None),
            (
                "2014-01-01T06:00:00.25",
                datetime.datetime(2014, 1, 1, 6, 0, 0, 250000),
            ),
            ("2014-01-01T06:00:00-05:00", None),
            ("2014-01-01T06:00:00x", None),
            ("2014-01-01T06:00", None),
            ("2014-01-01", None),
            ("Mon 1st Jan 2014 9 am", None),
            ("invalid", None),
            ("", None),
            ("2014-1-01T06:00:00", None),
            ("2014-01-01T6:00:00", None),
            (" 2014-01-01T06:00:00", None),
            ("+2014-01-01T06:00:00", None),
        ],
    )
    def test_default(self, cell: str, expected: datetime.datetime | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = LocalDateTimeColumn(
            name="name",
            type="local-date-time",
            property=LocalDateTimeColumnProperty(),
        )

        result = table.select(parse_local_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            (
                "21/11/2006 16:30",
                datetime.datetime(2006, 11, 21, 16, 30),
            ),
            ("21/11/2006 16:30:00", None),
            ("16:30 21/11/06", None),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_temporal_format(self, cell: str, expected: datetime.datetime | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = LocalDateTimeColumn(
            name="name",
            type="local-date-time",
            property=LocalDateTimeColumnProperty(
                format="local-date-time",
                temporalFormat="%d/%m/%Y %H:%M",
            ),
        )

        result = table.select(parse_local_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "fmt, cell, expected",
        [
            (
                "%Y-%m-%d %H:%M:%S",
                "2024-01-01 00:00:59",
                datetime.datetime(2024, 1, 1, 0, 0, 59),
            ),
            (
                "%Y-%m-%d %H:%M:%S",
                "2024-1-1 0:0:5",
                datetime.datetime(2024, 1, 1, 0, 0, 5),
            ),
            ("%Y-%m-%d %H:%M:%S", "2024-01-01 00:00:60", None),
            ("%Y-%m-%d %H:%M:%S", "2024-01-01 23:59:60", None),
            ("%Y%m%d%H%M%S", "20240101000059", datetime.datetime(2024, 1, 1, 0, 0, 59)),
            ("%Y%m%d%H%M%S", "20240101000060", None),
        ],
    )
    def test_temporal_format_seconds(
        self, fmt: str, cell: str, expected: datetime.datetime | None
    ):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = LocalDateTimeColumn(
            name="name",
            type="local-date-time",
            property=LocalDateTimeColumnProperty(temporalFormat=fmt),
        )

        result = table.select(parse_local_date_time_column(column, pl.col("name")))

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
        column = LocalDateTimeColumn(
            name="name",
            type="local-date-time",
            property=LocalDateTimeColumnProperty(temporalFormat="invalid"),
        )

        result = table.select(parse_local_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyLocalDateTimeColumn:
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
            (
                datetime.datetime(2006, 11, 21, 16, 30, 0, 125000),
                "2006-11-21T16:30:00.125",
            ),
        ],
    )
    def test_default(self, value: datetime.datetime, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Datetime)}).lazy()
        column = LocalDateTimeColumn(
            name="name",
            type="local-date-time",
            property=LocalDateTimeColumnProperty(),
        )

        result = table.select(stringify_local_date_time_column(column, pl.col("name")))

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
        column = LocalDateTimeColumn(
            name="name",
            type="local-date-time",
            property=LocalDateTimeColumnProperty(
                format="local-date-time",
                temporalFormat="%d/%m/%Y %H:%M",
            ),
        )

        result = table.select(stringify_local_date_time_column(column, pl.col("name")))

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
        column = LocalDateTimeColumn(
            name="name",
            type="local-date-time",
            property=LocalDateTimeColumnProperty(
                format="local-date-time",
                temporalFormat="%Y/%m/%dT%H:%M:%S",
            ),
        )

        result = table.select(stringify_local_date_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
