from __future__ import annotations

import polars as pl
from fairspec_metadata import BooleanColumnProperty, IntegerColumnProperty
from fairspec_metadata import NumberColumnProperty
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

    def test_should_skip_unique_keys_with_any_null_part(self):
        table = pl.DataFrame(
            {
                "a": [1, None, None, 1, None],
                "b": ["x", "y", "y", None, None],
            }
        ).lazy()
        table_schema = TableSchema(
            properties={
                "a": IntegerColumnProperty(type=("integer", "null")),
                "b": StringColumnProperty(type=("string", "null")),
            },
            uniqueKeys=[["a", "b"]],
        )

        assert inspect_table(table, table_schema=table_schema) == []

    def test_should_skip_unique_keys_with_a_missing_value_part(self):
        table = pl.DataFrame({"code": ["NA", "a", "NA"]}).lazy()
        table_schema = TableSchema(
            missingValues=["NA"],
            properties={"code": StringColumnProperty(type=("string", "null"))},
            uniqueKeys=[["code"]],
        )

        assert inspect_table(table, table_schema=table_schema) == []

    def test_should_report_null_primary_key_parts_as_missing_cells(self):
        table = pl.DataFrame(
            {"a": [None, None, 1], "b": [None, None, "z"]},
            schema={"a": pl.Int64, "b": pl.String},
        ).lazy()
        table_schema = TableSchema(
            properties={
                "a": IntegerColumnProperty(type=("integer", "null")),
                "b": StringColumnProperty(type=("string", "null")),
            },
            primaryKey=["a", "b"],
        )

        errors = inspect_table(table, table_schema=table_schema)

        dumps = [error.model_dump() for error in errors]
        assert [(d["type"], d["columnName"], d["rowNumber"]) for d in dumps] == [
            ("cell/missing", "a", 1),
            ("cell/missing", "a", 2),
            ("cell/missing", "b", 1),
            ("cell/missing", "b", 2),
        ]

    def test_should_check_a_key_column_named_like_the_probe_count(self):
        table = pl.DataFrame({"len": [1, 2, 1]}).lazy()
        table_schema = TableSchema(
            properties={"len": IntegerColumnProperty()},
            primaryKey=["len"],
        )

        errors = inspect_table(table, table_schema=table_schema)

        pk_errors = [e for e in errors if isinstance(e, RowPrimaryKeyError)]
        assert len(errors) == 1
        assert pk_errors[0].rowNumber == 3

    def test_should_cap_key_errors_per_check(self):
        table = pl.DataFrame({"id": [1] * 10, "name": ["a"] * 10}).lazy()
        table_schema = TableSchema(
            properties={"id": IntegerColumnProperty(), "name": StringColumnProperty()},
            primaryKey=["id"],
        )

        errors = inspect_table(table, table_schema=table_schema, max_errors=4)

        pk_errors = [e for e in errors if isinstance(e, RowPrimaryKeyError)]
        assert len(errors) == 2
        assert [error.rowNumber for error in pk_errors] == [2, 3]

    def test_should_find_a_duplicate_key_beside_cell_errors(self):
        table = pl.DataFrame({"id": ["1", "2", "x", "2"]}).lazy()
        table_schema = TableSchema(
            properties={"id": IntegerColumnProperty()},
            primaryKey=["id"],
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert [(error.type, getattr(error, "rowNumber", None)) for error in errors] == [
            ("cell/type", 3),
            ("row/primaryKey", 4),
        ]

    def test_should_find_a_duplicate_key_without_cell_checks(self):
        table = pl.DataFrame({"name": ["a", "b", "a"]}).lazy()
        table_schema = TableSchema(
            properties={"name": StringColumnProperty()},
            uniqueKeys=[["name"]],
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert [(error.type, getattr(error, "rowNumber", None)) for error in errors] == [
            ("row/uniqueKey", 3)
        ]

    def test_should_find_a_duplicate_key_across_read_batches(self):
        ids = [str(index) for index in range(300_000)] + ["0"]
        table = pl.DataFrame({"id": ids}).lazy()
        table_schema = TableSchema(
            properties={"id": IntegerColumnProperty()},
            primaryKey=["id"],
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert [(error.type, getattr(error, "rowNumber", None)) for error in errors] == [
            ("row/primaryKey", 300_001)
        ]

    def test_should_leave_key_errors_out_once_cell_errors_fill_the_report(self):
        table = pl.DataFrame({"id": ["x", "y", "1", "1"]}).lazy()
        table_schema = TableSchema(
            properties={"id": IntegerColumnProperty()},
            primaryKey=["id"],
        )

        errors = inspect_table(table, table_schema=table_schema, max_errors=2)

        assert [(error.type, getattr(error, "rowNumber", None)) for error in errors] == [
            ("cell/type", 1),
            ("cell/type", 2),
        ]


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


class TestInspectTableTypedKeys:
    def test_should_compare_numbers_as_values(self):
        table = pl.DataFrame({"id": ["1", "1.0", "2.5"]}).lazy()
        table_schema = TableSchema(
            properties={"id": NumberColumnProperty()}, primaryKey=["id"]
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == [
            RowPrimaryKeyError(type="row/primaryKey", rowNumber=2, columnNames=["id"])
        ]

    def test_should_find_repeated_keys_that_parse_to_one_literal(self, tmp_path):
        path = tmp_path / "table.csv"
        path.write_text("flag\nyes\ny\n")
        table = pl.scan_csv(path, infer_schema=False)
        table_schema = TableSchema(
            properties={"flag": BooleanColumnProperty(trueValues=["yes", "y"])},
            primaryKey=["flag"],
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert errors == [
            RowPrimaryKeyError(type="row/primaryKey", rowNumber=2, columnNames=["flag"])
        ]
