from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import IntegerColumn, IntegerColumnProperty

from ..integer import parse_integer_column, stringify_integer_column


class TestParseIntegerColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1", 1),
            ("2", 2),
            ("1000", 1000),
            ("0", 0),
            ("00", 0),
            ("01", 1),
            ("000835", 835),
            ("", None),
            ("2.1", None),
            ("bad", None),
            ("0.0003", None),
            ("3.14", None),
            ("1/2", None),
        ],
    )
    def test_default(self, cell: str, expected: int | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = IntegerColumn(
            name="name",
            type="integer",
            property=IntegerColumnProperty(),
        )

        result = table.select(parse_integer_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1", 1),
            ("1,000", 1000),
            ("1,000,000", 1000000),
            ("000,001", 1),
        ],
    )
    def test_group_char_comma(self, cell: str, expected: int):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = IntegerColumn(
            name="name",
            type="integer",
            property=IntegerColumnProperty(groupChar=","),
        )

        result = table.select(parse_integer_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, group_char, expected",
        [
            ("1 000", " ", 1000),
            ("1'000'000", "'", 1000000),
            ("1.000.000", ".", 1000000),
        ],
    )
    def test_group_char_other(self, cell: str, group_char: str, expected: int):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = IntegerColumn(
            name="name",
            type="integer",
            property=IntegerColumnProperty(groupChar=group_char),
        )

        result = table.select(parse_integer_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1", 1),
            ("1000", 1000),
            ("$1000", 1000),
            ("1000$", 1000),
            ("\u20ac1000", 1000),
            ("1000\u20ac", 1000),
            ("-12\u20ac", -12),
            ("\u20ac-12", -12),
            ("1,000", None),
        ],
    )
    def test_with_text(self, cell: str, expected: int | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = IntegerColumn(
            name="name",
            type="integer",
            property=IntegerColumnProperty(withText=True),
        )

        result = table.select(parse_integer_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("$1,000,000", 1000000),
            ("1,000,000$", 1000000),
        ],
    )
    def test_with_text_and_group_char(self, cell: str, expected: int):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = IntegerColumn(
            name="name",
            type="integer",
            property=IntegerColumnProperty(groupChar=",", withText=True),
        )

        result = table.select(parse_integer_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyIntegerColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (1, "1"),
            (2, "2"),
            (1000, "1000"),
            (42, "42"),
            (-1, "-1"),
            (-100, "-100"),
            (0, "0"),
            (1234567890, "1234567890"),
            (-1234567890, "-1234567890"),
        ],
    )
    def test_default(self, value: int, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Int64)}).lazy()
        column = IntegerColumn(
            name="name",
            type="integer",
            property=IntegerColumnProperty(),
        )

        result = table.select(stringify_integer_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
