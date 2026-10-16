from __future__ import annotations

from decimal import Decimal

import polars as pl
import pytest

from fairspec_metadata import DecimalColumn, DecimalColumnProperty

from ..decimal import parse_decimal_column, stringify_decimal_column


class TestParseDecimalColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1", Decimal("1")),
            ("2", Decimal("2")),
            ("1000", Decimal("1000")),
            ("1.5", Decimal("1.5")),
            ("4.14159", Decimal("4.14159")),
            ("-42", Decimal("-42")),
            ("-3.14", Decimal("-3.14")),
            ("", None),
            ("bad", None),
            ("text", None),
        ],
    )
    def test_default(self, cell: str, expected: Decimal | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DecimalColumn(
            name="name",
            type="decimal",
            property=DecimalColumnProperty(),
        )

        result = table.select(parse_decimal_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1", Decimal("1")),
            ("1,000", Decimal("1000")),
            ("1,000,000", Decimal("1000000")),
            ("1,234.56", Decimal("1234.56")),
        ],
    )
    def test_group_char_comma(self, cell: str, expected: Decimal):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DecimalColumn(
            name="name",
            type="decimal",
            property=DecimalColumnProperty(groupChar=","),
        )

        result = table.select(parse_decimal_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1,5", Decimal("1.5")),
            ("3,14", Decimal("3.14")),
        ],
    )
    def test_decimal_char_comma(self, cell: str, expected: Decimal):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DecimalColumn(
            name="name",
            type="decimal",
            property=DecimalColumnProperty(decimalChar=","),
        )

        result = table.select(parse_decimal_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1.234,56", Decimal("1234.56")),
            ("1.000,00", Decimal("1000.00")),
        ],
    )
    def test_group_char_dot_decimal_char_comma(self, cell: str, expected: Decimal):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DecimalColumn(
            name="name",
            type="decimal",
            property=DecimalColumnProperty(
                format="decimal", groupChar=".", decimalChar=","
            ),
        )

        result = table.select(parse_decimal_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("$1.5", Decimal("1.5")),
            ("1.5%", Decimal("1.5")),
            ("\u20ac1000", Decimal("1000")),
            ("1000\u20ac", Decimal("1000")),
        ],
    )
    def test_with_text(self, cell: str, expected: Decimal):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DecimalColumn(
            name="name",
            type="decimal",
            property=DecimalColumnProperty(withText=True),
        )

        result = table.select(parse_decimal_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("$1,000.00", Decimal("1000.00")),
            ("1,234.56$", Decimal("1234.56")),
        ],
    )
    def test_with_text_and_group_char(self, cell: str, expected: Decimal):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DecimalColumn(
            name="name",
            type="decimal",
            property=DecimalColumnProperty(
                format="decimal", withText=True, groupChar=","
            ),
        )

        result = table.select(parse_decimal_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("\u20ac 1.000,00", Decimal("1000.00")),
            ("1.000,00 \u20ac", Decimal("1000.00")),
            ("1.234,56 \u20ac", Decimal("1234.56")),
        ],
    )
    def test_with_text_group_char_dot_decimal_char_comma(
        self, cell: str, expected: Decimal
    ):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = DecimalColumn(
            name="name",
            type="decimal",
            property=DecimalColumnProperty(
                format="decimal",
                withText=True,
                groupChar=".",
                decimalChar=",",
            ),
        )

        result = table.select(parse_decimal_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyDecimalColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (Decimal("1.0"), "1.0"),
            (Decimal("2.0"), "2.0"),
            (Decimal("1000.0"), "1000.0"),
            (Decimal("3.14"), "3.14"),
            (Decimal("42.5"), "42.5"),
            (Decimal("-1.0"), "-1.0"),
            (Decimal("-100.5"), "-100.5"),
            (Decimal("0.0"), "0.0"),
        ],
    )
    def test_default(self, value: Decimal, expected: str):
        table = pl.DataFrame({"name": pl.Series([value], dtype=pl.Decimal)}).lazy()
        column = DecimalColumn(
            name="name",
            type="decimal",
            property=DecimalColumnProperty(),
        )

        result = table.select(stringify_decimal_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
