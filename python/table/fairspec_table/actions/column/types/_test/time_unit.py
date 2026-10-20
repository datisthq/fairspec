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
            ("06:00:00Z", datetime.time(6, 0, 0)),
            ("06:00:00+02:00", datetime.time(4, 0, 0)),
            ("01:00:00+02:00", datetime.time(23, 0, 0)),
            ("06:00:00.5-05:00", datetime.time(11, 0, 0, 500000)),
            ("06:00:00", None),
            ("06:00:00+0200", None),
            ("09:00Z", None),
            ("invalid", None),
            ("", None),
        ],
    )
    def test_default(self, cell: str, expected: datetime.time | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = TimeColumn(name="name", type="time", property=TimeColumnProperty())

        result = table.select(parse_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("06:00+02:00", datetime.time(4, 0, 0)),
            ("06:00+0200", datetime.time(4, 0, 0)),
            ("06:00", None),
            ("invalid", None),
        ],
    )
    def test_temporal_format(self, cell: str, expected: datetime.time | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = TimeColumn(
            name="name",
            type="time",
            property=TimeColumnProperty(temporalFormat="%H:%M%:z"),
        )

        result = table.select(parse_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyTimeColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (datetime.time(6, 0, 0), "06:00:00Z"),
            (datetime.time(16, 30, 0, 500000), "16:30:00.500Z"),
        ],
    )
    def test_default(self, value: datetime.time, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Time)}).lazy()
        column = TimeColumn(name="name", type="time", property=TimeColumnProperty())

        result = table.select(stringify_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    def test_temporal_format(self):
        table = pl.DataFrame(
            {"name": pl.Series([datetime.time(16, 30)], dtype=pl.Time)}
        ).lazy()
        column = TimeColumn(
            name="name",
            type="time",
            property=TimeColumnProperty(temporalFormat="%H:%M%z"),
        )

        result = table.select(stringify_time_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": "16:30+0000"}]
