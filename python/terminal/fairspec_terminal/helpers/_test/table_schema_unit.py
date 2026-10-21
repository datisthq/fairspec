from __future__ import annotations

import pytest

from fairspec_terminal.helpers.table_schema import create_infer_table_schema_options
from fairspec_terminal.params import TableSchemaParams
from fairspec_terminal.params.table_schema import (
    parse_column_properties,
    parse_column_types,
)


class TestCreateInferTableSchemaOptions:
    def test_should_leave_unset_options_out(self):
        assert create_infer_table_schema_options(TableSchemaParams()) == {}

    def test_should_pass_option_values(self):
        params = TableSchemaParams(
            sample_rows=10,
            keep_strings=True,
            column_types={"id": "integer", "name": "string"},
            missing_values=["NA", "-"],
            array_type="list",
            list_item_type="integer",
        )
        assert create_infer_table_schema_options(params) == {
            "sampleRows": 10,
            "keepStrings": True,
            "columnTypes": {"id": "integer", "name": "string"},
            "missingValues": ["NA", "-"],
            "arrayType": "list",
            "listItemType": "integer",
        }


class TestParseColumnTypes:
    def test_should_parse_name_type_pairs(self):
        assert parse_column_types("id:integer,name:string") == {
            "id": "integer",
            "name": "string",
        }


class TestParseColumnProperties:
    def test_should_parse_json_object(self):
        assert parse_column_properties(
            '{"id": {"type": "string"}, "name": {"title": "Name"}}'
        ) == {"id": {"type": "string"}, "name": {"title": "Name"}}

    def test_should_reject_json_that_is_not_an_object(self):
        with pytest.raises(ValueError, match="must be a JSON object"):
            parse_column_properties("[1, 2]")
