from __future__ import annotations

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
