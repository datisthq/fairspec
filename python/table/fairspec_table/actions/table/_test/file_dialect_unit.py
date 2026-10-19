from __future__ import annotations

import polars as pl
from fairspec_metadata import CsvFileDialect

from ..file_dialect import join_header_rows, skip_comment_rows


class TestJoinHeaderRows:
    def test_should_join_two_header_rows_with_default_space_separator(self):
        table = pl.DataFrame(
            {
                "column_1": ["first", "name", "Alice", "Bob"],
                "column_2": ["last", "name", "Smith", "Jones"],
                "column_3": ["contact", "email", "alice@example.com", "bob@example.com"],
            }
        ).lazy()

        result = join_header_rows(table, CsvFileDialect(headerRows=[1, 2]))

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == ["first name", "last name", "contact email"]
        assert collected.rows() == [
            ("Alice", "Smith", "alice@example.com"),
            ("Bob", "Jones", "bob@example.com"),
        ]

    def test_should_join_two_header_rows_with_custom_separator(self):
        table = pl.DataFrame(
            {
                "column_1": ["user", "first", "Alice"],
                "column_2": ["user", "last", "Smith"],
                "column_3": ["meta", "created", "2023-01-01"],
            }
        ).lazy()

        result = join_header_rows(
            table, CsvFileDialect(headerRows=[1, 2], headerJoin="_")
        )

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == ["user_first", "user_last", "meta_created"]
        assert collected.rows() == [("Alice", "Smith", "2023-01-01")]

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

    def test_should_join_three_header_rows_after_skipped_rows(self):
        table = pl.DataFrame(
            {
                "column_1": ["person", "user", "first", "Alice"],
                "column_2": ["person", "user", "last", "Smith"],
                "column_3": ["location", "address", "city", "NYC"],
            }
        ).lazy()

        result = join_header_rows(table, CsvFileDialect(headerRows=[2, 3, 4]))

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == [
            "person user first",
            "person user last",
            "location address city",
        ]
        assert collected.rows() == [("Alice", "Smith", "NYC")]

    def test_should_carry_a_label_forward_across_empty_header_cells(self):
        table = pl.DataFrame(
            {
                "column_1": ["person", "first", "Alice"],
                "column_2": [None, "last", "Smith"],
                "column_3": ["location", "city", "NYC"],
            }
        ).lazy()

        result = join_header_rows(table, CsvFileDialect(headerRows=[1, 2]))

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == ["person first", "person last", "location city"]
        assert collected.rows() == [("Alice", "Smith", "NYC")]

    def test_should_drop_the_other_header_rows_with_column_names(self):
        table = pl.DataFrame(
            {
                "key": ["id", "1"],
                "label": ["name", "apple"],
            }
        ).lazy()

        result = join_header_rows(
            table, CsvFileDialect(headerRows=[1, 2], columnNames=["key", "label"])
        )

        collected: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert collected.columns == ["key", "label"]
        assert collected.rows() == [("1", "apple")]


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
