from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import NumberColumn, NumberColumnProperty

from ..number import parse_number_column, stringify_number_column


class TestParseNumberColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1", 1.0),
            ("2", 2.0),
            ("1000", 1000.0),
            ("1.5", 1.5),
            ("4.14159", 4.14159),
            ("-42", -42.0),
            ("-3.14", -3.14),
            ("", None),
            ("bad", None),
            ("text", None),
        ],
    )
    def test_default(self, cell: str, expected: float | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = NumberColumn(
            name="name",
            type="number",
            property=NumberColumnProperty(),
        )

        result = table.select(parse_number_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1", 1.0),
            ("1,000", 1000.0),
            ("1,000,000", 1000000.0),
            ("1,234.56", 1234.56),
        ],
    )
    def test_group_char_comma(self, cell: str, expected: float):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = NumberColumn(
            name="name",
            type="number",
            property=NumberColumnProperty(groupChar=","),
        )

        result = table.select(parse_number_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1,5", 1.5),
            ("3,14", 3.14),
        ],
    )
    def test_decimal_char_comma(self, cell: str, expected: float):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = NumberColumn(
            name="name",
            type="number",
            property=NumberColumnProperty(decimalChar=","),
        )

        result = table.select(parse_number_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1.234,56", 1234.56),
            ("1.000,00", 1000.0),
        ],
    )
    def test_group_char_dot_decimal_char_comma(self, cell: str, expected: float):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = NumberColumn(
            name="name",
            type="number",
            property=NumberColumnProperty(groupChar=".", decimalChar=","),
        )

        result = table.select(parse_number_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("$1.5", 1.5),
            ("1.5%", 1.5),
            ("\u20ac1000", 1000.0),
            ("1000\u20ac", 1000.0),
        ],
    )
    def test_with_text(self, cell: str, expected: float):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = NumberColumn(
            name="name",
            type="number",
            property=NumberColumnProperty(withText=True),
        )

        result = table.select(parse_number_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("$1,000.00", 1000.0),
            ("1,234.56$", 1234.56),
        ],
    )
    def test_with_text_and_group_char(self, cell: str, expected: float):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = NumberColumn(
            name="name",
            type="number",
            property=NumberColumnProperty(withText=True, groupChar=","),
        )

        result = table.select(parse_number_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("\u20ac 1.000,00", 1000.0),
            ("1.000,00 \u20ac", 1000.0),
            ("1.234,56 \u20ac", 1234.56),
        ],
    )
    def test_with_text_group_char_dot_decimal_char_comma(
        self, cell: str, expected: float
    ):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = NumberColumn(
            name="name",
            type="number",
            property=NumberColumnProperty(withText=True, groupChar=".", decimalChar=","),
        )

        result = table.select(parse_number_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyNumberColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (1.0, "1.0"),
            (2.0, "2.0"),
            (1000.0, "1000.0"),
            (3.14, "3.14"),
            (42.5, "42.5"),
            (-1.0, "-1.0"),
            (-100.5, "-100.5"),
            (0.0, "0.0"),
            (-123.456789, "-123.456789"),
            (1234567890.123, "1234567890.123"),
            (-9876543210.987, "-9876543210.987"),
            (0.001, "0.001"),
            (-0.0001, "-0.0001"),
        ],
    )
    def test_default(self, value: float, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Float64)}).lazy()
        column = NumberColumn(
            name="name",
            type="number",
            property=NumberColumnProperty(),
        )

        result = table.select(stringify_number_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
