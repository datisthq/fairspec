from __future__ import annotations

import polars as pl
from fairspec_metadata import BooleanColumnProperty
from fairspec_metadata import DateColumnProperty
from fairspec_metadata import DateTimeColumnProperty
from fairspec_metadata import IntegerColumnProperty
from fairspec_metadata import NumberColumnProperty
from fairspec_metadata import StringColumnProperty
from fairspec_metadata import TimeColumnProperty
from fairspec_metadata import CellTypeError
from fairspec_metadata import ColumnMissingError, ColumnTypeError
from fairspec_metadata import TableSchema

from fairspec_table.actions.table.inspect import inspect_table


class TestInspectColumnName:
    def test_should_report_error_when_column_names_dont_match(self):
        table = pl.DataFrame({"actual_id": [1, 2, 3]}).lazy()
        table_schema = TableSchema(
            allRequired=True, properties={"id": NumberColumnProperty()}
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 1
        assert isinstance(errors[0], ColumnMissingError)
        assert errors[0].columnName == "id"

    def test_should_not_error_when_column_names_match(self):
        table = pl.DataFrame({"id": [1, 2, 3]}).lazy()
        table_schema = TableSchema(properties={"id": NumberColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 0

    def test_should_be_case_sensitive_when_comparing_column_names(self):
        table = pl.DataFrame({"ID": [1, 2, 3]}).lazy()
        table_schema = TableSchema(
            allRequired=True, properties={"id": NumberColumnProperty()}
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 1
        assert isinstance(errors[0], ColumnMissingError)
        assert errors[0].columnName == "id"


class TestInspectColumnType:
    def test_should_report_error_when_column_types_dont_match(self):
        table = pl.DataFrame({"id": [True, False, True]}).lazy()
        table_schema = TableSchema(properties={"id": IntegerColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 1
        assert isinstance(errors[0], ColumnTypeError)
        assert errors[0].columnName == "id"
        assert errors[0].expectedColumnType == "integer"
        assert errors[0].actualColumnType == "boolean"

    def test_should_not_error_when_column_types_match(self):
        table = pl.DataFrame({"id": [1, 2, 3]}).lazy()
        table_schema = TableSchema(properties={"id": NumberColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 0


class TestInspectColumnNamedNumber:
    def test_should_not_crash_when_data_has_column_named_number(self):
        table = pl.DataFrame(
            {"name": ["Alice", "Bob", "Charlie"], "number": [1, 2, 3]}
        ).lazy()
        table_schema = TableSchema(
            properties={
                "name": StringColumnProperty(),
                "number": IntegerColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 0

    def test_should_report_cell_errors_with_correct_row_numbers_for_column_named_number(
        self,
    ):
        table = pl.DataFrame(
            {"name": ["Alice", "Bob", "Charlie"], "number": ["1", "bad", "3"]}
        ).lazy()
        table_schema = TableSchema(
            properties={
                "name": StringColumnProperty(),
                "number": IntegerColumnProperty(),
            }
        )

        errors = inspect_table(table, table_schema=table_schema)

        cell_errors = [e for e in errors if isinstance(e, CellTypeError)]
        assert len(cell_errors) == 1
        assert cell_errors[0].cell == "bad"
        assert cell_errors[0].columnName == "number"
        assert cell_errors[0].rowNumber == 2


class TestInspectCellTypes:
    def test_should_validate_string_to_integer_conversion_errors(self):
        table = pl.DataFrame({"id": ["1", "bad", "3", "4x"]}).lazy()
        table_schema = TableSchema(properties={"id": IntegerColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 2
        assert isinstance(errors[0], CellTypeError)
        assert errors[0].cell == "bad"
        assert errors[0].columnName == "id"
        assert errors[0].columnType == "integer"
        assert errors[0].rowNumber == 2
        assert isinstance(errors[1], CellTypeError)
        assert errors[1].cell == "4x"
        assert errors[1].rowNumber == 4

    def test_should_validate_string_to_number_conversion_errors(self):
        table = pl.DataFrame({"price": ["10.5", "twenty", "30.75", "$40"]}).lazy()
        table_schema = TableSchema(properties={"price": NumberColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 2
        assert isinstance(errors[0], CellTypeError)
        assert errors[0].cell == "twenty"
        assert errors[0].columnName == "price"
        assert errors[0].columnType == "number"
        assert errors[0].rowNumber == 2
        assert isinstance(errors[1], CellTypeError)
        assert errors[1].cell == "$40"
        assert errors[1].rowNumber == 4

    def test_should_validate_string_to_boolean_conversion_errors(self):
        table = pl.DataFrame({"active": ["true", "yes", "false", "0", "1"]}).lazy()
        table_schema = TableSchema(properties={"active": BooleanColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 1
        assert isinstance(errors[0], CellTypeError)
        assert errors[0].cell == "yes"
        assert errors[0].columnName == "active"
        assert errors[0].columnType == "boolean"
        assert errors[0].rowNumber == 2

    def test_should_validate_string_to_date_conversion_errors(self):
        table = pl.DataFrame(
            {"created": ["2023-01-15", "Jan 15, 2023", "20230115", "not-a-date"]}
        ).lazy()
        table_schema = TableSchema(properties={"created": DateColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        cell_errors = [e for e in errors if isinstance(e, CellTypeError)]
        assert len(cell_errors) == 3
        assert cell_errors[0].cell == "Jan 15, 2023"
        assert cell_errors[0].rowNumber == 2
        assert cell_errors[1].cell == "20230115"
        assert cell_errors[1].rowNumber == 3
        assert cell_errors[2].cell == "not-a-date"
        assert cell_errors[2].rowNumber == 4

    def test_should_validate_string_to_time_conversion_errors(self):
        table = pl.DataFrame({"time": ["14:30:00", "2:30pm", "invalid", "14h30"]}).lazy()
        table_schema = TableSchema(properties={"time": TimeColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        cell_errors = [e for e in errors if isinstance(e, CellTypeError)]
        assert len(cell_errors) == 3
        assert cell_errors[0].cell == "2:30pm"
        assert cell_errors[0].rowNumber == 2
        assert cell_errors[1].cell == "invalid"
        assert cell_errors[1].rowNumber == 3
        assert cell_errors[2].cell == "14h30"
        assert cell_errors[2].rowNumber == 4

    def test_should_validate_string_to_datetime_conversion_errors(self):
        table = pl.DataFrame(
            {
                "timestamp": [
                    "2023-01-15T14:30:00",
                    "January 15, 2023 2:30 PM",
                    "2023-01-15 14:30",
                    "not-a-datetime",
                ]
            }
        ).lazy()
        table_schema = TableSchema(properties={"timestamp": DateTimeColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) > 0
        cell_errors = [e for e in errors if isinstance(e, CellTypeError)]
        assert len(cell_errors) >= 2
        cells = {e.cell for e in cell_errors}
        assert "January 15, 2023 2:30 PM" in cells
        assert "not-a-datetime" in cells

    def test_should_pass_validation_when_all_cells_are_valid(self):
        table = pl.DataFrame({"id": ["1", "2", "3", "4"]}).lazy()
        table_schema = TableSchema(properties={"id": IntegerColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 0

    def test_should_validate_with_non_string_source_data(self):
        table = pl.DataFrame({"is_active": [True, False, True, False]}).lazy()
        table_schema = TableSchema(properties={"is_active": BooleanColumnProperty()})

        errors = inspect_table(table, table_schema=table_schema)

        assert len(errors) == 0
