from __future__ import annotations

import polars as pl

from fairspec_metadata import (
    ArrayColumn,
    ArrayColumnProperty,
    DurationColumn,
    DurationColumnProperty,
    ObjectColumn,
    ObjectColumnProperty,
)

from ..helpers import inspect_json_column, inspect_text_column


class TestInspectTextColumn:
    def test_no_errors_for_valid_values(self):
        table = pl.DataFrame({"duration": ["P1Y", "P2M", "P3D"]}).lazy()
        column = DurationColumn(
            name="duration",
            type="duration",
            property=DurationColumnProperty(),
        )

        errors = inspect_text_column(
            column, table, parse=lambda s: s if s.startswith("P") else None
        )

        assert errors == []

    def test_error_for_invalid_values(self):
        table = pl.DataFrame({"duration": ["P1Y", "invalid", "P3D"]}).lazy()
        column = DurationColumn(
            name="duration",
            type="duration",
            property=DurationColumnProperty(),
        )

        errors = inspect_text_column(
            column, table, parse=lambda s: s if s.startswith("P") else None
        )

        assert len(errors) == 1
        assert errors[0].type == "cell/type"
        assert errors[0].cell == "invalid"
        assert errors[0].columnName == "duration"
        assert errors[0].rowNumber == 2

    def test_skip_null_values(self):
        table = pl.DataFrame({"duration": ["P1Y", None, "P3D"]}).lazy()
        column = DurationColumn(
            name="duration",
            type="duration",
            nullable=True,
            property=DurationColumnProperty(),
        )

        errors = inspect_text_column(
            column, table, parse=lambda s: s if s.startswith("P") else None
        )

        assert errors == []

    def test_error_for_missing_values_in_non_null_column(self):
        table = pl.DataFrame({"duration": ["P1Y", None, "NA"]}).lazy()
        column = DurationColumn(
            name="duration",
            type="duration",
            property=DurationColumnProperty(missingValues=["NA"]),
        )

        errors = inspect_text_column(
            column, table, parse=lambda s: s if s.startswith("P") else None
        )

        assert [error.model_dump(exclude_none=True) for error in errors] == [
            {
                "type": "cell/missing",
                "columnName": "duration",
                "rowNumber": 2,
                "cell": "",
            },
            {
                "type": "cell/missing",
                "columnName": "duration",
                "rowNumber": 3,
                "cell": "NA",
            },
        ]

    def test_error_when_parse_raises(self):
        def failing_parse(source: str) -> object:
            raise ValueError("parse error")

        table = pl.DataFrame({"duration": ["bad"]}).lazy()
        column = DurationColumn(
            name="duration",
            type="duration",
            property=DurationColumnProperty(),
        )

        errors = inspect_text_column(column, table, parse=failing_parse)

        assert len(errors) == 1
        assert errors[0].type == "cell/type"


class TestInspectJsonColumn:
    def test_no_errors_for_valid_object(self):
        table = pl.DataFrame({"data": ['{"a": 1}']}).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_json_column(column, table)

        assert errors == []

    def test_error_for_invalid_json(self):
        table = pl.DataFrame({"data": ["not json"]}).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_json_column(column, table)

        assert len(errors) == 1
        assert errors[0].type == "cell/type"
        assert errors[0].cell == "not json"

    def test_error_for_array_when_expecting_object(self):
        table = pl.DataFrame({"data": ["[1, 2, 3]"]}).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_json_column(column, table)

        assert len(errors) == 1
        assert errors[0].type == "cell/type"

    def test_no_errors_for_valid_array(self):
        table = pl.DataFrame({"data": ["[1, 2, 3]"]}).lazy()
        column = ArrayColumn(
            name="data",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_json_column(column, table)

        assert errors == []

    def test_error_for_object_when_expecting_array(self):
        table = pl.DataFrame({"data": ['{"a": 1}']}).lazy()
        column = ArrayColumn(
            name="data",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_json_column(column, table)

        assert len(errors) == 1
        assert errors[0].type == "cell/type"

    def test_skip_null_values(self):
        table = pl.DataFrame({"data": [None, '{"a": 1}']}).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            nullable=True,
            property=ObjectColumnProperty(),
        )

        errors = inspect_json_column(column, table)

        assert errors == []

    def test_error_for_null_values_in_non_null_column(self):
        table = pl.DataFrame({"data": [None, '{"a": 1}']}).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_json_column(column, table)

        assert [error.model_dump(exclude_none=True) for error in errors] == [
            {"type": "cell/missing", "columnName": "data", "rowNumber": 1, "cell": ""}
        ]

    def test_type_json_schema_valid(self):
        table = pl.DataFrame({"data": ['{"name": "test"}']}).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_json_column(
            column,
            table,
            type_json_schema={
                "type": "object",
                "properties": {"name": {"type": "string"}},
                "required": ["name"],
            },
        )

        assert errors == []

    def test_type_json_schema_invalid(self):
        table = pl.DataFrame({"data": ['{"name": 123}']}).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_json_column(
            column,
            table,
            type_json_schema={
                "type": "object",
                "properties": {"name": {"type": "string"}},
                "required": ["name"],
            },
        )

        assert len(errors) == 1
        assert errors[0].type == "cell/type"
