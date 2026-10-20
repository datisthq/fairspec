from __future__ import annotations

import polars as pl
from fairspec_metadata import (
    CheckError,
    IntegerColumnProperty,
    RowCheckError,
    TableCheckError,
    TableSchema,
)

from ...table.inspect import inspect_table
from ..inspect import inspect_table_checks

table = pl.DataFrame(
    {
        "id": [1, 2, 3],
        "zip": ["a", "a", "b"],
        "start": [1, 3, 1],
        "end": [2, 2, 5],
        "price": [10, None, -1],
    }
).lazy()


class TestInspectTableChecks:
    def test_should_report_rows_failing_a_sql_condition(self):
        errors = inspect_table_checks(
            table,
            [{"name": "end-after-start", "condition": '"end" >= start'}],
            max_errors=100,
        )

        assert errors == [
            RowCheckError(type="row/check", checkName="end-after-start", rowNumber=2)
        ]

    def test_should_report_rows_failing_a_polars_condition(self):
        errors = inspect_table_checks(
            table,
            [{"name": "end-after-start", "condition": pl.col("end") >= pl.col("start")}],
            max_errors=100,
        )

        assert errors == [
            RowCheckError(type="row/check", checkName="end-after-start", rowNumber=2)
        ]

    def test_should_pass_rows_where_the_condition_is_null(self):
        errors = inspect_table_checks(
            table,
            [{"name": "positive-price", "condition": "price > 0"}],
            max_errors=100,
        )

        assert errors == [
            RowCheckError(type="row/check", checkName="positive-price", rowNumber=3)
        ]

    def test_should_report_a_cell_when_the_check_names_a_column(self):
        errors = inspect_table_checks(
            table,
            [
                {
                    "name": "positive-price",
                    "condition": pl.col("price") > 0,
                    "columnName": "price",
                    "message": "Price must be positive",
                }
            ],
            max_errors=100,
        )

        assert [error.model_dump(exclude_none=True) for error in errors] == [
            {
                "type": "cell/check",
                "checkName": "positive-price",
                "columnName": "price",
                "rowNumber": 3,
                "cell": "-1",
                "message": "Price must be positive",
            }
        ]

    def test_should_report_the_table_when_an_aggregate_condition_fails(self):
        errors = inspect_table_checks(
            table,
            [
                {"name": "sql-size", "condition": "count(*) >= 5"},
                {"name": "polars-size", "condition": pl.len() >= 5},
                {"name": "sql-sum", "condition": "sum(price) > 0"},
                {"name": "polars-sum", "condition": pl.col("price").sum() > 0},
            ],
            max_errors=100,
        )

        assert errors == [
            TableCheckError(type="table/check", checkName="sql-size"),
            TableCheckError(type="table/check", checkName="polars-size"),
        ]

    def test_should_check_rows_against_their_group_with_a_window(self):
        errors = inspect_table_checks(
            table,
            [
                {"name": "sql-zip", "condition": "count(*) OVER (PARTITION BY zip) >= 2"},
                {"name": "polars-zip", "condition": pl.len().over("zip") >= 2},
            ],
            max_errors=100,
        )

        assert errors == [
            RowCheckError(type="row/check", checkName="sql-zip", rowNumber=3),
            RowCheckError(type="row/check", checkName="polars-zip", rowNumber=3),
        ]

    def test_should_report_checks_that_cannot_be_evaluated(self):
        errors = inspect_table_checks(
            table,
            [
                {"name": "sql-column", "condition": "missing > 1"},
                {"name": "polars-column", "condition": pl.col("missing") > 1},
                {"name": "sql-syntax", "condition": "price >"},
                {"name": "not-boolean", "condition": "price + 1"},
                {
                    "name": "cell-column",
                    "condition": "price > 0",
                    "columnName": "missing",
                },
                {"name": "valid", "condition": "price > 0"},
            ],
            max_errors=100,
        )

        assert [(error.type, getattr(error, "checkName")) for error in errors] == [
            ("check", "sql-column"),
            ("check", "polars-column"),
            ("check", "sql-syntax"),
            ("check", "not-boolean"),
            ("check", "cell-column"),
            ("row/check", "valid"),
        ]
        assert errors[3] == CheckError(
            type="check",
            checkName="not-boolean",
            message="Condition is Int64 instead of Boolean",
        )
        assert errors[4] == CheckError(
            type="check",
            checkName="cell-column",
            message='Column "missing" is not found',
        )

    def test_should_stop_at_the_maximum_number_of_failing_rows(self):
        errors = inspect_table_checks(
            table,
            [{"name": "never", "condition": pl.lit(False) & pl.col("id").is_not_null()}],
            max_errors=2,
        )

        assert errors == [
            RowCheckError(type="row/check", checkName="never", rowNumber=1),
            RowCheckError(type="row/check", checkName="never", rowNumber=2),
        ]


class TestInspectTableWithChecks:
    def test_should_run_conditions_on_values_typed_by_the_schema(self):
        source = pl.DataFrame(
            {"id": ["1", "2", "3"], "amount": ["9", "10", "bad"]}
        ).lazy()
        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "amount": IntegerColumnProperty(),
            }
        )

        errors = inspect_table(
            source,
            table_schema=table_schema,
            checks=[
                {
                    "name": "small-amount",
                    "condition": "amount < 10",
                    "columnName": "amount",
                }
            ],
        )

        assert [error.model_dump(exclude_none=True) for error in errors] == [
            {
                "type": "cell/type",
                "columnName": "amount",
                "columnType": "integer",
                "rowNumber": 3,
                "cell": "bad",
            },
            {
                "type": "cell/check",
                "checkName": "small-amount",
                "columnName": "amount",
                "rowNumber": 2,
                "cell": "10",
            },
        ]

    def test_should_run_checks_without_a_schema(self):
        errors = inspect_table(
            table,
            checks=[{"name": "end-after-start", "condition": '"end" >= start'}],
        )

        assert errors == [
            RowCheckError(type="row/check", checkName="end-after-start", rowNumber=2)
        ]
