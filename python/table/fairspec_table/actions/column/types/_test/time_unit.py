from __future__ import annotations

import datetime

import polars as pl
import pytest

from fairspec_metadata import TimeColumn, TimeColumnProperty

from ..time import parse_time_column, stringify_time_column


class TestParseTimeColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("06:00:00", datetime.time(6, 0, 0)),
            ("09:00", None),
            ("3 am", None),
            ("3.00", None),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_default(self, cell: str, expected: datetime.time | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = TimeColumn(
            name="name",
            type="time",
            property=TimeColumnProperty(),
        )

        result = table.select(parse_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("06:00", datetime.time(6, 0, 0)),
            ("06:50", datetime.time(6, 50, 0)),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_temporal_format_hm(self, cell: str, expected: datetime.time | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = TimeColumn(
            name="name",
            type="time",
            property=TimeColumnProperty(temporalFormat="%H:%M"),
        )

        result = table.select(parse_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyTimeColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (datetime.time(6, 0, 0), "06:00:00"),
            (datetime.time(16, 30, 0), "16:30:00"),
        ],
    )
    def test_default(self, value: datetime.time, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Time)}).lazy()
        column = TimeColumn(
            name="name",
            type="time",
            property=TimeColumnProperty(),
        )

        result = table.select(stringify_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (datetime.time(6, 0, 0), "06:00"),
            (datetime.time(16, 30, 0), "16:30"),
        ],
    )
    def test_temporal_format_hm(self, value: datetime.time, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Time)}).lazy()
        column = TimeColumn(
            name="name",
            type="time",
            property=TimeColumnProperty(temporalFormat="%H:%M"),
        )

        result = table.select(stringify_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
