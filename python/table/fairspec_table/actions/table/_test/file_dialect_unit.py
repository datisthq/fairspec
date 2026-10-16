from __future__ import annotations

import polars as pl
from fairspec_metadata import CsvFileDialect

from ..file_dialect import join_header_rows, skip_comment_rows


class TestJoinHeaderRows:
    def test_should_join_two_header_rows_with_default_space_separator(self):
        table = pl.DataFrame(
            {
                "col1": ["first", "name", "header3", "Alice", "Bob"],
                "col2": ["last", "name", "header3", "Smith", "Jones"],
                "col3": [
                    "contact",
                    "email",
                    "header3",
                    "alice@example.com",
                    "bob@example.com",
                ],
            }
        ).lazy()

        result = join_header_rows(table, CsvFileDialect(headerRows=[2, 3]))

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == ["col1 first", "col2 last", "col3 contact"]
        assert collected.height == 4
        assert collected.row(0) == ("name", "name", "email")
        assert collected.row(1) == ("header3", "header3", "header3")
        assert collected.row(2) == ("Alice", "Smith", "alice@example.com")
        assert collected.row(3) == ("Bob", "Jones", "bob@example.com")

    def test_should_join_two_header_rows_with_custom_separator(self):
        table = pl.DataFrame(
            {
                "col1": ["user", "first", "header3", "Alice", "Bob"],
                "col2": ["user", "last", "header3", "Smith", "Jones"],
                "col3": ["meta", "created", "header3", "2023-01-01", "2023-01-02"],
            }
        ).lazy()

        result = join_header_rows(
            table, CsvFileDialect(headerRows=[2, 3], headerJoin="_")
        )

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == ["col1_user", "col2_user", "col3_meta"]
        assert collected.height == 4
        assert collected.row(0) == ("first", "last", "created")
        assert collected.row(1) == ("header3", "header3", "header3")
        assert collected.row(2) == ("Alice", "Smith", "2023-01-01")
        assert collected.row(3) == ("Bob", "Jones", "2023-01-02")

    def test_should_return_table_unchanged_when_only_one_header_row(self):
        table = pl.DataFrame(
            {
                "name": ["Alice", "Bob"],
                "age": [30, 25],
                "city": ["NYC", "LA"],
            }
        ).lazy()

        result = join_header_rows(table, CsvFileDialect(headerRows=[1]))

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == ["name", "age", "city"]
        assert collected.height == 2

    def test_should_return_table_unchanged_when_no_header_rows(self):
        table = pl.DataFrame(
            {
                "field1": ["Alice", "Bob"],
                "field2": [30, 25],
                "field3": ["NYC", "LA"],
            }
        ).lazy()

        result = join_header_rows(table, CsvFileDialect())

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == ["field1", "field2", "field3"]
        assert collected.height == 2

    def test_should_join_three_header_rows(self):
        table = pl.DataFrame(
            {
                "col1": ["person", "user", "first", "header4", "Alice", "Bob"],
                "col2": ["person", "user", "last", "header4", "Smith", "Jones"],
                "col3": ["location", "address", "city", "header4", "NYC", "LA"],
            }
        ).lazy()

        result = join_header_rows(table, CsvFileDialect(headerRows=[2, 3, 4]))

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == [
            "col1 person user",
            "col2 person user",
            "col3 location address",
        ]
        assert collected.height == 4
        assert collected.row(0) == ("first", "last", "city")
        assert collected.row(1) == ("header4", "header4", "header4")
        assert collected.row(2) == ("Alice", "Smith", "NYC")
        assert collected.row(3) == ("Bob", "Jones", "LA")

    def test_should_handle_empty_strings_in_header_rows(self):
        table = pl.DataFrame(
            {
                "col1": ["person", "", "header3", "Alice", "Bob"],
                "col2": ["", "name", "header3", "Smith", "Jones"],
                "col3": ["location", "city", "header3", "NYC", "LA"],
            }
        ).lazy()

        result = join_header_rows(table, CsvFileDialect(headerRows=[2, 3]))

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == ["col1 person", "col2 ", "col3 location"]
        assert collected.height == 4
        assert collected.row(0) == ("", "name", "city")
        assert collected.row(1) == ("header3", "header3", "header3")
        assert collected.row(2) == ("Alice", "Smith", "NYC")
        assert collected.row(3) == ("Bob", "Jones", "LA")


class TestSkipCommentRows:
    def test_should_skip_comment_rows_by_row_number(self):
        table = pl.DataFrame(
            {
                "name": ["Alice", "# Comment", "Bob", "Charlie"],
                "age": [30, 0, 25, 35],
                "city": ["NYC", "ignored", "LA", "SF"],
            }
        ).lazy()

        result = skip_comment_rows(
            table, CsvFileDialect(commentRows=[2], headerRows=False)
        )

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.height == 3
        assert collected.row(0) == ("Alice", 30, "NYC")
        assert collected.row(1) == ("Bob", 25, "LA")
        assert collected.row(2) == ("Charlie", 35, "SF")

    def test_should_skip_multiple_comment_rows(self):
        table = pl.DataFrame(
            {
                "name": ["Alice", "# Comment 1", "Bob", "# Comment 2", "Charlie"],
                "age": [30, 0, 25, 0, 35],
                "city": ["NYC", "ignored", "LA", "ignored", "SF"],
            }
        ).lazy()

        result = skip_comment_rows(
            table, CsvFileDialect(commentRows=[2, 4], headerRows=False)
        )

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.height == 3
        assert collected.row(0) == ("Alice", 30, "NYC")
        assert collected.row(1) == ("Bob", 25, "LA")
        assert collected.row(2) == ("Charlie", 35, "SF")

    def test_should_return_table_unchanged_when_no_comment_rows_specified(self):
        table = pl.DataFrame(
            {
                "name": ["Alice", "Bob", "Charlie"],
                "age": [30, 25, 35],
                "city": ["NYC", "LA", "SF"],
            }
        ).lazy()

        result = skip_comment_rows(table, CsvFileDialect())

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.height == 3
        assert collected.columns == ["name", "age", "city"]

    def test_should_skip_rows_after_header_when_header_rows_specified(self):
        table = pl.DataFrame(
            {
                "col1": ["name", "Alice", "# Comment", "Bob"],
                "col2": ["age", "30", "-1", "25"],
                "col3": ["city", "NYC", "ignored", "LA"],
            }
        ).lazy()

        result = skip_comment_rows(table, CsvFileDialect(headerRows=[2], commentRows=[5]))

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.height == 3
        assert collected.row(0) == ("name", "age", "city")
        assert collected.row(1) == ("Alice", "30", "NYC")
        assert collected.row(2) == ("Bob", "25", "LA")

    def test_should_handle_comment_rows_at_the_beginning(self):
        table = pl.DataFrame(
            {
                "name": ["# Skip this", "Alice", "Bob"],
                "age": [0, 30, 25],
                "city": ["ignored", "NYC", "LA"],
            }
        ).lazy()

        result = skip_comment_rows(
            table, CsvFileDialect(commentRows=[1], headerRows=False)
        )

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.height == 2
        assert collected.row(0) == ("Alice", 30, "NYC")
        assert collected.row(1) == ("Bob", 25, "LA")

    def test_should_handle_comment_rows_at_the_end(self):
        table = pl.DataFrame(
            {
                "name": ["Alice", "Bob", "# Footer comment"],
                "age": [30, 25, 0],
                "city": ["NYC", "LA", "ignored"],
            }
        ).lazy()

        result = skip_comment_rows(
            table, CsvFileDialect(commentRows=[3], headerRows=False)
        )

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.height == 2
        assert collected.row(0) == ("Alice", 30, "NYC")
        assert collected.row(1) == ("Bob", 25, "LA")

    def test_should_handle_multiple_header_rows_with_comment_rows(self):
        table = pl.DataFrame(
            {
                "col1": ["person", "first", "Alice", "# Comment", "Bob"],
                "col2": ["person", "last", "Smith", "ignored", "Jones"],
                "col3": ["location", "city", "NYC", "ignored", "LA"],
            }
        ).lazy()

        result = skip_comment_rows(
            table, CsvFileDialect(headerRows=[2, 3], commentRows=[7])
        )

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.height == 4
        assert collected.row(0) == ("person", "person", "location")
        assert collected.row(1) == ("first", "last", "city")
        assert collected.row(2) == ("Alice", "Smith", "NYC")
        assert collected.row(3) == ("Bob", "Jones", "LA")
