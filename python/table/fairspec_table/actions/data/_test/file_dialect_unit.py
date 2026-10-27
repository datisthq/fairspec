from __future__ import annotations

import pytest

from fairspec_metadata import CsvFileDialect

from ..file_dialect import get_records_from_rows


class TestGetRecordsFromRows:
    def test_convert_rows_to_records_with_default_header(self):
        rows = [
            ["name", "age", "city"],
            ["Alice", 30, "NYC"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(rows)

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ]

    def test_single_row_with_header(self):
        rows: list[list[object]] = [["name", "age", "city"]]

        result = get_records_from_rows(rows)

        assert result == []

    def test_empty_rows(self):
        rows: list[list[object]] = []

        with pytest.raises(Exception, match="past the end"):
            get_records_from_rows(rows)

    def test_empty_rows_without_header(self):
        rows: list[list[object]] = []

        result = get_records_from_rows(rows, CsvFileDialect(headerRows=False))

        assert result == []

    def test_rows_between_header_rows_are_dropped(self):
        rows: list[list[object]] = [["a", "b"], ["skip", "me"], ["c", "d"], [1, 2]]

        result = get_records_from_rows(rows, CsvFileDialect(headerRows=[1, 3]))

        assert result == [{"a c": 1, "b d": 2}]

    def test_repeated_header_name(self):
        rows: list[list[object]] = [["a", "a"], [1, 2]]

        with pytest.raises(Exception, match="repeated"):
            get_records_from_rows(rows)

    def test_empty_header_cell_is_an_empty_name(self):
        rows: list[list[object]] = [["a", None], [1, 2]]

        result = get_records_from_rows(rows)

        assert result == [{"a": 1, "": 2}]

    def test_rows_without_header_when_header_is_false(self):
        rows = [
            ["Alice", 30, "NYC"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(headerRows=False),
        )

        assert result == [
            {"column1": "Alice", "column2": 30, "column3": "NYC"},
            {"column1": "Bob", "column2": 25, "column3": "LA"},
        ]

    def test_custom_header_rows(self):
        rows = [
            ["skip this row"],
            ["name", "age", "city"],
            ["Alice", 30, "NYC"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(headerRows=[2]),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ]

    def test_multiple_header_rows_with_default_join(self):
        rows = [
            ["first", "last", "contact"],
            ["name", "name", "email"],
            ["Alice", "Smith", "alice@example.com"],
            ["Bob", "Jones", "bob@example.com"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(headerRows=[1, 2]),
        )

        assert result == [
            {
                "first name": "Alice",
                "last name": "Smith",
                "contact email": "alice@example.com",
            },
            {
                "first name": "Bob",
                "last name": "Jones",
                "contact email": "bob@example.com",
            },
        ]

    def test_multiple_header_rows_with_custom_join(self):
        rows = [
            ["user", "user", "meta"],
            ["first", "last", "created"],
            ["Alice", "Smith", "2023-01-01"],
            ["Bob", "Jones", "2023-01-02"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(headerRows=[1, 2], headerJoin="_"),
        )

        assert result == [
            {"user_first": "Alice", "user_last": "Smith", "meta_created": "2023-01-01"},
            {"user_first": "Bob", "user_last": "Jones", "meta_created": "2023-01-02"},
        ]

    def test_skip_comment_rows_by_row_number(self):
        rows = [
            ["name", "age", "city"],
            ["Alice", 30, "NYC"],
            ["# Comment row", "ignored", "data"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(commentRows=[3]),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ]

    def test_skip_rows_with_comment_character(self):
        rows = [
            ["name", "age", "city"],
            ["Alice", 30, "NYC"],
            ["# Comment", "ignored", "data"],
            ["Bob", 25, "LA"],
            ["Regular row", "data", "value"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(commentPrefix="#"),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
            {"name": "Regular row", "age": "data", "city": "value"},
        ]

    def test_skip_rows_with_multiple_comment_characters(self):
        rows = [
            ["name", "age", "city"],
            ["Alice", 30, "NYC"],
            ["# Comment 1", "ignored", "data"],
            ["Bob", 25, "LA"],
            ["## Comment 2", "ignored", "data"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(commentPrefix="#"),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ]

    def test_not_skip_rows_when_first_cell_is_not_string(self):
        rows = [
            ["name", "age", "city"],
            ["Alice", 30, "NYC"],
            [123, "data", "test"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(commentPrefix="#"),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": 123, "age": "data", "city": "test"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ]

    def test_rows_with_different_lengths(self):
        rows = [
            ["name", "age", "city", "country"],
            ["Alice", 30, "NYC"],
            ["Bob", 25, "LA", "USA"],
            ["Charlie"],
        ]

        result = get_records_from_rows(rows)

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC", "country": None},
            {"name": "Bob", "age": 25, "city": "LA", "country": "USA"},
            {"name": "Charlie", "age": None, "city": None, "country": None},
        ]

    def test_null_values(self):
        rows = [
            ["name", "age", "city"],
            ["Alice", None, None],
            [None, 25, "LA"],
        ]

        result = get_records_from_rows(rows)

        assert result == [
            {"name": "Alice", "age": None, "city": None},
            {"name": None, "age": 25, "city": "LA"},
        ]

    def test_boolean_and_number_types(self):
        rows = [
            ["name", "active", "count"],
            ["Alice", True, 100],
            ["Bob", False, 0],
        ]

        result = get_records_from_rows(rows)

        assert result == [
            {"name": "Alice", "active": True, "count": 100},
            {"name": "Bob", "active": False, "count": 0},
        ]

    def test_convert_header_values_to_strings(self):
        rows = [
            [1, 2, 3],
            ["Alice", 30, "NYC"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(rows)

        assert result == [
            {"1": "Alice", "2": 30, "3": "NYC"},
            {"1": "Bob", "2": 25, "3": "LA"},
        ]

    def test_empty_header_cells(self):
        rows = [
            ["name", "", "city"],
            ["Alice", 30, "NYC"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(rows)

        assert result == [
            {"name": "Alice", "": 30, "city": "NYC"},
            {"name": "Bob", "": 25, "city": "LA"},
        ]

    def test_multi_row_headers_with_empty_cells(self):
        rows = [
            ["person", "", "location"],
            ["first", "last", "city"],
            ["Alice", "Smith", "NYC"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(headerRows=[1, 2]),
        )

        assert result == [
            {"person first": "Alice", "person last": "Smith", "location city": "NYC"},
        ]

    def test_combination_of_header_rows_and_comment_rows(self):
        rows = [
            ["skip row 1"],
            ["name", "age", "city"],
            ["# Comment", "data", "data"],
            ["Alice", 30, "NYC"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(headerRows=[2], commentRows=[3]),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ]

    def test_combination_of_comment_rows_and_comment_prefix(self):
        rows = [
            ["name", "age", "city"],
            ["# Inline comment", "data", "data"],
            ["Alice", 30, "NYC"],
            ["Comment by row number", "data", "data"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(commentRows=[4], commentPrefix="#"),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ]

    def test_generate_column_names_based_on_longest_row_when_no_header(self):
        rows = [
            ["Alice", 30],
            ["Bob", 25, "LA", "USA"],
            ["Charlie", 35, "SF"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(headerRows=False),
        )

        assert result == [
            {"column1": "Alice", "column2": 30, "column3": None, "column4": None},
            {"column1": "Bob", "column2": 25, "column3": "LA", "column4": "USA"},
            {"column1": "Charlie", "column2": 35, "column3": "SF", "column4": None},
        ]

    def test_use_column_names_when_provided_with_header_rows_false(self):
        rows = [
            ["Alice", 30, "NYC"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(
                format="csv",
                headerRows=False,
                columnNames=["name", "age", "city"],
            ),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ]

    def test_use_column_names_even_when_file_has_headers(self):
        rows = [
            ["firstName", "years", "location"],
            ["Alice", 30, "NYC"],
            ["Bob", 25, "LA"],
        ]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(columnNames=["name", "age", "city"]),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": "NYC"},
            {"name": "Bob", "age": 25, "city": "LA"},
        ]

    def test_column_names_with_rows_longer_than_column_names_array(self):
        rows: list[list[object]] = [
            ["Alice", 30, "NYC", "USA"],
            ["Bob", 25, "LA", "USA"],
        ]

        with pytest.raises(Exception, match="columnNames"):
            get_records_from_rows(
                rows,
                CsvFileDialect(
                    format="csv",
                    headerRows=False,
                    columnNames=["name", "age"],
                ),
            )

    def test_column_names_with_rows_shorter_than_column_names_array(self):
        rows: list[list[object]] = [["Alice", 30], ["Bob"]]

        result = get_records_from_rows(
            rows,
            CsvFileDialect(
                format="csv",
                headerRows=False,
                columnNames=["name", "age", "city", "country"],
            ),
        )

        assert result == [
            {"name": "Alice", "age": 30, "city": None, "country": None},
            {"name": "Bob", "age": None, "city": None, "country": None},
        ]
