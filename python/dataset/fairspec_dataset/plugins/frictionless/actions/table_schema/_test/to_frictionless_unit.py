from ..to_frictionless import convert_table_schema_to_frictionless


class TestConvertTableSchemaToFrictionless:
    def test_requires_a_column_that_does_not_allow_null(self):
        schema = convert_table_schema_to_frictionless(
            {
                "properties": {
                    "id": {"type": "integer"},
                    "name": {"type": ["string", "null"]},
                }
            }
        )

        assert schema["fields"] == [
            {"name": "id", "type": "integer", "constraints": {"required": True}},
            {"name": "name", "type": "string"},
        ]

    def test_keeps_an_empty_cell_missing(self):
        assert "missingValues" not in convert_table_schema_to_frictionless(
            {"properties": {}}
        )
        assert convert_table_schema_to_frictionless(
            {"properties": {}, "missingValues": ["NA"]}
        )["missingValues"] == ["NA", ""]

    def test_writes_a_self_reference_as_an_empty_resource(self):
        schema = convert_table_schema_to_frictionless(
            {
                "properties": {"id": {"type": "integer"}, "parent": {"type": "integer"}},
                "foreignKeys": [
                    {"columns": ["parent"], "reference": {"columns": ["id"]}}
                ],
            }
        )

        assert schema["foreignKeys"] == [
            {"fields": ["parent"], "reference": {"resource": "", "fields": ["id"]}}
        ]
