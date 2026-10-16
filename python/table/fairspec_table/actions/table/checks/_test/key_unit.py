from __future__ import annotations

import polars as pl
from fairspec_metadata import IntegerColumnProperty
from fairspec_metadata import StringColumnProperty
from fairspec_metadata import RowPrimaryKeyError, RowUniqueKeyError
from fairspec_metadata import TableSchema

from fairspec_table.actions.table.inspect import inspect_table


class TestInspectTableRowUnique:
    def test_should_not_error_when_all_rows_are_unique_for_primary_key(self):
        table = pl.DataFrame(
            {
                "id": [1, 2, 3, 4, 5],
                "name": ["Alice", "Bob", "Charlie", "David", "Eve"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            },
            primaryKey=["id"],
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 0

    def test_should_error_for_duplicate_primary_key_rows(self):
        table = pl.DataFrame(
            {
                "id": [1, 2, 3, 2, 5],
                "name": ["Alice", "Bob", "Charlie", "Bob2", "Eve"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            },
            primaryKey=["id"],
        )

        errors = inspect_table(table, table_schema=table_schema)

        pk_errors = [e for e in errors if isinstance(e, RowPrimaryKeyError)]
        assert len(pk_errors) == 1
        assert pk_errors[0].rowNumber == 4
        assert pk_errors[0].columnNames == ["id"]

    def test_should_not_error_when_all_rows_are_unique_for_unique_key(self):
        table = pl.DataFrame(
            {
                "id": [1, 2, 3, 4, 5],
                "email": [
                    "a@test.com",
                    "b@test.com",
                    "c@test.com",
                    "d@test.com",
                    "e@test.com",
                ],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "email": StringColumnProperty(),
            },
            uniqueKeys=[["email"]],
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 0

    def test_should_error_for_duplicate_unique_key_rows(self):
        table = pl.DataFrame(
            {
                "id": [1, 2, 3, 4, 5],
                "email": [
                    "a@test.com",
                    "b@test.com",
                    "a@test.com",
                    "d@test.com",
                    "b@test.com",
                ],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "email": StringColumnProperty(),
            },
            uniqueKeys=[["email"]],
        )

        errors = inspect_table(table, table_schema=table_schema)

        uk_errors = [e for e in errors if isinstance(e, RowUniqueKeyError)]
        assert len(uk_errors) == 2
        assert uk_errors[0].rowNumber == 3
        assert uk_errors[0].columnNames == ["email"]
        assert uk_errors[1].rowNumber == 5
        assert uk_errors[1].columnNames == ["email"]

    def test_should_handle_composite_unique_keys(self):
        table = pl.DataFrame(
            {
                "category": ["A", "A", "B", "A", "B"],
                "subcategory": ["X", "Y", "X", "X", "Y"],
                "value": [1, 2, 3, 4, 5],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "category": StringColumnProperty(),
                "subcategory": StringColumnProperty(),
                "value": IntegerColumnProperty(),
            },
            uniqueKeys=[["category", "subcategory"]],
        )

        errors = inspect_table(table, table_schema=table_schema)

        uk_errors = [e for e in errors if isinstance(e, RowUniqueKeyError)]
        assert len(uk_errors) == 1
        assert uk_errors[0].rowNumber == 4
        assert uk_errors[0].columnNames == ["category", "subcategory"]

    def test_should_handle_both_primary_key_and_unique_keys(self):
        table = pl.DataFrame(
            {
                "id": [1, 2, 3, 2, 5],
                "email": [
                    "a@test.com",
                    "b@test.com",
                    "c@test.com",
                    "d@test.com",
                    "a@test.com",
                ],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "email": StringColumnProperty(),
            },
            primaryKey=["id"],
            uniqueKeys=[["email"]],
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 2
        pk_errors = [e for e in errors if isinstance(e, RowPrimaryKeyError)]
        uk_errors = [e for e in errors if isinstance(e, RowUniqueKeyError)]
        assert len(pk_errors) == 1
        assert pk_errors[0].rowNumber == 4
        assert pk_errors[0].columnNames == ["id"]
        assert len(uk_errors) == 1
        assert uk_errors[0].rowNumber == 5
        assert uk_errors[0].columnNames == ["email"]

    def test_should_handle_null_values_in_unique_keys_correctly(self):
        table = pl.DataFrame(
            {
                "id": [1, 2, None, 4, None, 2],
                "name": ["Alice", "Bob", "Charlie", "David", "Eve", "Bob"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(type=("integer", "null")),
                "name": StringColumnProperty(),
            },
            uniqueKeys=[["id"], ["id", "name"]],
        )

        errors = inspect_table(table, table_schema=table_schema)

        uk_errors = [e for e in errors if isinstance(e, RowUniqueKeyError)]
        assert len(uk_errors) == 2
        assert uk_errors[0].rowNumber == 6
        assert uk_errors[0].columnNames == ["id"]
        assert uk_errors[1].rowNumber == 6
        assert uk_errors[1].columnNames == ["id", "name"]


class TestInspectTableRowChecksConcurrency:
    def test_should_keep_row_check_order_under_concurrency(self):
        table = pl.DataFrame(
            {
                "id": [1, 1, 2],
                "email": ["a@a", "a@a", "b@b"],
                "code": ["x", "x", "y"],
            }
        ).lazy()
        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "email": StringColumnProperty(),
                "code": StringColumnProperty(),
            },
            primaryKey=["id"],
            uniqueKeys=[["email"], ["code"]],
        )

        serial = inspect_table(table, table_schema=table_schema, concurrency=1)
        concurrent = inspect_table(table, table_schema=table_schema, concurrency=4)

        assert [error.type for error in serial] == [
            "row/primaryKey",
            "row/uniqueKey",
            "row/uniqueKey",
        ]
        assert [error.model_dump() for error in serial] == [
            error.model_dump() for error in concurrent
        ]
