from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import BooleanColumn, BooleanColumnProperty

from ..boolean import parse_boolean_column, stringify_boolean_column


class TestParseBooleanColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("true", True),
            ("True", True),
            ("TRUE", True),
            ("1", True),
            ("false", False),
            ("False", False),
            ("FALSE", False),
            ("0", False),
            ("", None),
            ("invalid", None),
            ("truthy", None),
            ("falsy", None),
            ("2", None),
            ("-100", None),
            ("t", None),
            ("f", None),
            ("3.14", None),
        ],
    )
    def test_default(self, cell: str, expected: bool | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = BooleanColumn(
            name="name",
            type="boolean",
            property=BooleanColumnProperty(),
        )

        result = table.select(parse_boolean_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("Y", True),
            ("y", True),
            ("yes", True),
            ("true", None),
        ],
    )
    def test_true_values(self, cell: str, expected: bool | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = BooleanColumn(
            name="name",
            type="boolean",
            property=BooleanColumnProperty(
                trueValues=["Y", "y", "yes"],
            ),
        )

        result = table.select(parse_boolean_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("N", False),
            ("n", False),
            ("no", False),
            ("false", None),
        ],
    )
    def test_false_values(self, cell: str, expected: bool | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = BooleanColumn(
            name="name",
            type="boolean",
            property=BooleanColumnProperty(
                falseValues=["N", "n", "no"],
            ),
        )

        result = table.select(parse_boolean_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("oui", True),
            ("si", True),
            ("non", False),
            ("no", False),
        ],
    )
    def test_true_values_and_false_values(self, cell: str, expected: bool | None):
        table = pl.DataFrame({"name": [cell]}).lazy()
        column = BooleanColumn(
            name="name",
            type="boolean",
            property=BooleanColumnProperty(
                trueValues=["oui", "si"],
                falseValues=["non", "no"],
            ),
        )

        result = table.select(parse_boolean_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]


class TestStringifyBooleanColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (True, "true"),
            (False, "false"),
        ],
    )
    def test_default(self, value: bool, expected: str):
        table = pl.DataFrame({"name": [value]}).lazy()
        column = BooleanColumn(
            name="name",
            type="boolean",
            property=BooleanColumnProperty(),
        )

        result = table.select(stringify_boolean_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (True, "Y"),
            (False, "false"),
        ],
    )
    def test_true_values(self, value: bool, expected: str):
        table = pl.DataFrame({"name": [value]}).lazy()
        column = BooleanColumn(
            name="name",
            type="boolean",
            property=BooleanColumnProperty(
                trueValues=["Y", "y", "yes"],
            ),
        )

        result = table.select(stringify_boolean_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (True, "true"),
            (False, "N"),
        ],
    )
    def test_false_values(self, value: bool, expected: str):
        table = pl.DataFrame({"name": [value]}).lazy()
        column = BooleanColumn(
            name="name",
            type="boolean",
            property=BooleanColumnProperty(
                falseValues=["N", "n", "no"],
            ),
        )

        result = table.select(stringify_boolean_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]

    @pytest.mark.parametrize(
        "value, expected",
        [
            (True, "oui"),
            (False, "non"),
        ],
    )
    def test_true_values_and_false_values(self, value: bool, expected: str):
        table = pl.DataFrame({"name": [value]}).lazy()
        column = BooleanColumn(
            name="name",
            type="boolean",
            property=BooleanColumnProperty(
                trueValues=["oui", "si"],
                falseValues=["non", "no"],
            ),
        )

        result = table.select(stringify_boolean_column(column, pl.col("name")))

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": expected}]
