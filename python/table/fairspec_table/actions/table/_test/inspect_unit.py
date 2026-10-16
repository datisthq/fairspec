from __future__ import annotations

import polars as pl
from fairspec_metadata import NumberColumnProperty
from fairspec_metadata import StringColumnProperty
from fairspec_metadata import ColumnMissingError
from fairspec_metadata import TableSchema

from ..inspect import inspect_table


class TestInspectTable:
    def test_should_pass_when_columns_exactly_match(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "name": ["John", "Jane"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == []

    def test_should_not_have_columns_error_when_columns_same_length(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "age": [30, 25],
            }
        ).lazy()

        table_schema = TableSchema(
            allRequired=True,
            properties={
                "id": NumberColumnProperty(),
                "name": NumberColumnProperty(),
            },
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 1
        assert isinstance(errors[0], ColumnMissingError)
        assert errors[0].columnName == "name"

    def test_should_detect_missing_columns(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            allRequired=True,
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            },
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 1
        assert isinstance(errors[0], ColumnMissingError)
        assert errors[0].columnName == "name"

    def test_should_pass_when_column_names_match_regardless_of_order(self):
        table = pl.DataFrame(
            {
                "name": ["John", "Jane"],
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == []

    def test_should_detect_missing_columns_with_required(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            required=["name"],
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            },
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 1
        assert isinstance(errors[0], ColumnMissingError)
        assert errors[0].columnName == "name"

    def test_should_pass_when_non_required_columns_are_missing(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == []

    def test_should_pass_when_data_contains_all_schema_columns(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "name": ["John", "Jane"],
                "age": [30, 25],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == []

    def test_should_pass_when_data_contains_exact_schema_columns(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "name": ["John", "Jane"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == []

    def test_should_detect_missing_columns_again(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            required=["name"],
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            },
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 1
        assert isinstance(errors[0], ColumnMissingError)
        assert errors[0].columnName == "name"

    def test_should_pass_when_schema_contains_all_data_columns(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == []

    def test_should_pass_when_schema_contains_exact_data_columns(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "name": ["John", "Jane"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == []

    def test_should_pass_when_at_least_one_column_matches(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "age": [30, 25],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == []

    def test_should_detect_missing_columns_with_all_required(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            allRequired=True,
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            },
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 1
        assert isinstance(errors[0], ColumnMissingError)
        assert errors[0].columnName == "name"

    def test_should_detect_when_no_columns_match(self):
        table = pl.DataFrame(
            {
                "age": [30, 25],
                "email": ["john@example.com", "jane@example.com"],
            }
        ).lazy()

        table_schema = TableSchema(
            allRequired=True,
            properties={
                "id": NumberColumnProperty(),
                "name": StringColumnProperty(),
            },
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 2
        assert isinstance(errors[0], ColumnMissingError)
        assert errors[0].columnName == "id"
        assert isinstance(errors[1], ColumnMissingError)
        assert errors[1].columnName == "name"


class TestInspectTableConcurrency:
    def _create_table(self) -> pl.LazyFrame:
        return pl.DataFrame(
            {f"c{index}": ["BAD", "ALSO BAD"] for index in range(8)}
        ).lazy()

    def _create_table_schema(self) -> TableSchema:
        from fairspec_metadata import IntegerColumnProperty

        return TableSchema(
            properties={f"c{index}": IntegerColumnProperty() for index in range(8)}
        )

    def test_should_produce_identical_errors_for_serial_and_concurrent(self):
        table = self._create_table()
        table_schema = self._create_table_schema()

        serial = inspect_table(table, table_schema=table_schema, concurrency=1)
        concurrent = inspect_table(table, table_schema=table_schema, concurrency=4)

        assert [error.model_dump() for error in serial] == [
            error.model_dump() for error in concurrent
        ]

    def test_should_truncate_identically_with_max_errors(self):
        table = self._create_table()
        table_schema = self._create_table_schema()

        serial = inspect_table(
            table, table_schema=table_schema, max_errors=5, concurrency=1
        )
        concurrent = inspect_table(
            table, table_schema=table_schema, max_errors=5, concurrency=4
        )

        assert len(serial) == 5
        assert [error.model_dump() for error in serial] == [
            error.model_dump() for error in concurrent
        ]

    def test_should_keep_missing_column_errors_in_schema_order(self):
        table = pl.DataFrame({"b": [1], "d": [1]}).lazy()
        table_schema = TableSchema(
            properties={
                "a": StringColumnProperty(),
                "b": StringColumnProperty(),
                "c": StringColumnProperty(),
                "d": StringColumnProperty(),
                "e": StringColumnProperty(),
            },
            allRequired=True,
        )

        errors = inspect_table(table, table_schema=table_schema, concurrency=4)
        missing = [
            error.columnName for error in errors if isinstance(error, ColumnMissingError)
        ]

        assert missing == ["a", "c", "e"]
