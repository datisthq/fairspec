from __future__ import annotations

import polars as pl
import pytest

from fairspec_metadata import ListColumn, ListColumnProperty


from ..list import parse_list_column, stringify_list_column


class TestParseListColumn:
    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("a,b,c", ["a", "b", "c"]),
            ("1,2,3", ["1", "2", "3"]),
            ("foo,bar,baz", ["foo", "bar", "baz"]),
            ("single", ["single"]),
            ("a,,c", ["a", "", "c"]),
            (",b,", ["", "b", ""]),
            (",,,", ["", "", "", ""]),
        ],
    )
    def test_default(self, cell: str, expected: list[str]):
        table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
        column = ListColumn(
            name="name",
            type="list",
            property=ListColumnProperty(),
        )

        result = table.select(parse_list_column(column, pl.col("name")).alias("name"))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        actual = frame.to_dicts()[0]["name"]

        assert actual == expected

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1,2,3", [1, 2, 3]),
            ("0,-1,42", [0, -1, 42]),
            ("-10,0,10", [-10, 0, 10]),
            ("42", [42]),
            ("1,,3", [1, None, 3]),
            (",2,", [None, 2, None]),
            ("1,a,3", [1, None, 3]),
            ("1.5,2,3", [None, 2, 3]),
        ],
    )
    def test_items_integer(self, cell: str, expected: list[int | None]):
        table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
        column = ListColumn(
            name="name",
            type="list",
            property=ListColumnProperty(itemType="integer"),  # ty: ignore[invalid-argument-type] https://github.com/astral-sh/ty/issues/2403
        )

        result = table.select(parse_list_column(column, pl.col("name")).alias("name"))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        actual = frame.to_dicts()[0]["name"]

        assert actual == expected

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("1.5,2.1,3.7", [1.5, 2.1, 3.7]),
            ("0,-1.1,42", [0.0, -1.1, 42.0]),
            ("-10.5,0,10", [-10.5, 0.0, 10.0]),
            ("3.14", [3.14]),
            ("1.1,,3.3", [1.1, None, 3.3]),
            (",2.2,", [None, 2.2, None]),
            ("1.1,a,3.3", [1.1, None, 3.3]),
        ],
    )
    def test_items_number(self, cell: str, expected: list[float | None]):
        table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
        column = ListColumn(
            name="name",
            type="list",
            property=ListColumnProperty(itemType="number"),  # ty: ignore[invalid-argument-type] https://github.com/astral-sh/ty/issues/2403
        )

        result = table.select(parse_list_column(column, pl.col("name")).alias("name"))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        actual = frame.to_dicts()[0]["name"]

        assert actual == expected

    @pytest.mark.parametrize(
        "cell, expected",
        [
            ("a;b;c", ["a", "b", "c"]),
            ("1;2;3", ["1", "2", "3"]),
            ("single", ["single"]),
            ("a;;c", ["a", "", "c"]),
        ],
    )
    def test_delimiter_semicolon(self, cell: str, expected: list[str]):
        table = pl.DataFrame([pl.Series("name", [cell], pl.String)]).lazy()
        column = ListColumn(
            name="name",
            type="list",
            property=ListColumnProperty(delimiter=";"),
        )

        result = table.select(parse_list_column(column, pl.col("name")).alias("name"))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        actual = frame.to_dicts()[0]["name"]

        assert actual == expected


class TestStringifyListColumn:
    @pytest.mark.parametrize(
        "value, expected",
        [
            (["a", "b", "c"], "a,b,c"),
            (["foo", "bar", "baz"], "foo,bar,baz"),
            (["1", "2", "3"], "1,2,3"),
            (["single"], "single"),
            (["a", "", "c"], "a,,c"),
            (["", "b", ""], ",b,"),
            (["", "", "", ""], ",,,"),
            ([None, "b", None], "b"),
            (["a", None, "c"], "a,c"),
            ([], ""),
        ],
    )
    def test_default(self, value: list[str | None], expected: str):
        table = pl.DataFrame([pl.Series("name", [value], pl.List(pl.String))]).lazy()
        column = ListColumn(
            name="name",
            type="list",
            property=ListColumnProperty(),
        )

        result = table.select(stringify_list_column(column, pl.col("name")).alias("name"))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        actual = frame.to_dicts()[0]["name"]

        assert actual == expected

    @pytest.mark.parametrize(
        "value, expected",
        [
            ([1, 2, 3], "1,2,3"),
            ([0, -1, 42], "0,-1,42"),
            ([-10, 0, 10], "-10,0,10"),
            ([42], "42"),
            ([1, None, 3], "1,3"),
            ([None, 2, None], "2"),
            ([], ""),
        ],
    )
    def test_items_integer(self, value: list[int | None], expected: str):
        table = pl.DataFrame([pl.Series("name", [value], pl.List(pl.Int16))]).lazy()
        column = ListColumn(
            name="name",
            type="list",
            property=ListColumnProperty(itemType="integer"),  # ty: ignore[invalid-argument-type] https://github.com/astral-sh/ty/issues/2403
        )

        result = table.select(stringify_list_column(column, pl.col("name")).alias("name"))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        actual = frame.to_dicts()[0]["name"]

        assert actual == expected

    @pytest.mark.parametrize(
        "value, expected",
        [
            ([1.5, 2.1, 3.7], "1.5,2.1,3.7"),
            ([0.0, -1.1, 42.0], "0.0,-1.1,42.0"),
            ([-10.5, 0.0, 10.0], "-10.5,0.0,10.0"),
            ([3.14], "3.14"),
            ([1.1, None, 3.3], "1.1,3.3"),
            ([None, 2.2, None], "2.2"),
            ([], ""),
        ],
    )
    def test_items_number(self, value: list[float | None], expected: str):
        table = pl.DataFrame([pl.Series("name", [value], pl.List(pl.Float64))]).lazy()
        column = ListColumn(
            name="name",
            type="list",
            property=ListColumnProperty(itemType="number"),  # ty: ignore[invalid-argument-type] https://github.com/astral-sh/ty/issues/2403
        )

        result = table.select(stringify_list_column(column, pl.col("name")).alias("name"))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        actual = frame.to_dicts()[0]["name"]

        assert actual == expected

    @pytest.mark.parametrize(
        "value, expected",
        [
            (["a", "b", "c"], "a;b;c"),
            (["1", "2", "3"], "1;2;3"),
            (["single"], "single"),
            (["a", "", "c"], "a;;c"),
            (["", "b", ""], ";b;"),
            ([], ""),
        ],
    )
    def test_delimiter_semicolon(self, value: list[str], expected: str):
        table = pl.DataFrame([pl.Series("name", [value], pl.List(pl.String))]).lazy()
        column = ListColumn(
            name="name",
            type="list",
            property=ListColumnProperty(delimiter=";"),
        )

        result = table.select(stringify_list_column(column, pl.col("name")).alias("name"))
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        actual = frame.to_dicts()[0]["name"]

        assert actual == expected
