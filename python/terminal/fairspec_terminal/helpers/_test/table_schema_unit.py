from __future__ import annotations

import pytest

from fairspec_terminal.helpers.table_schema import create_infer_table_schema_options


class TestCreateInferTableSchemaOptions:
    def test_should_leave_unset_options_out(self):
        assert create_infer_table_schema_options() == {}

    def test_should_parse_option_values(self):
        options = create_infer_table_schema_options(
            sample_rows=10,
            keep_strings=True,
            column_types="id:integer,name:string",
            missing_values="NA,-",
            array_type="list",
            list_item_type="integer",
        )
        assert options == {
            "sampleRows": 10,
            "keepStrings": True,
            "columnTypes": {"id": "integer", "name": "string"},
            "missingValues": ["NA", "-"],
            "arrayType": "list",
            "listItemType": "integer",
        }

    def test_should_parse_column_properties_json(self):
        options = create_infer_table_schema_options(
            column_properties='{"id": {"type": "string"}, "name": {"title": "Name"}}'
        )
        assert options == {
            "columnProperties": {"id": {"type": "string"}, "name": {"title": "Name"}}
        }

    def test_should_reject_column_properties_that_are_not_an_object(self):
        with pytest.raises(ValueError, match="must be a JSON object"):
            create_infer_table_schema_options(column_properties="[1, 2]")
