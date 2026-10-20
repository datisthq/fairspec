from fairspec_library import Resource, write_temp_file

from ..infer import infer_table_schema_tool


class TestInferTableSchemaTool:
    def test_should_infer_schema_from_csv(self):
        path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")

        result = infer_table_schema_tool(Resource(data=path))

        assert result.tableSchema is not None
        dump = result.model_dump(mode="json", by_alias=True)
        assert dump["tableSchema"]["properties"] == {
            "id": {"type": "integer"},
            "name": {"type": "string"},
        }

    def test_should_forward_options(self):
        path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")

        result = infer_table_schema_tool(
            Resource(data=path), {"columnTypes": {"id": "string"}}
        )

        dump = result.model_dump(mode="json", by_alias=True)
        assert dump["tableSchema"]["properties"]["id"] == {"type": "string"}

    def test_should_forward_column_properties(self):
        path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")

        result = infer_table_schema_tool(
            Resource(data=path),
            {"columnProperties": {"id": {"type": "string"}, "name": {"title": "Name"}}},
        )

        dump = result.model_dump(mode="json", by_alias=True)
        assert dump["tableSchema"]["properties"]["id"] == {"type": "string"}
        assert dump["tableSchema"]["properties"]["name"] == {
            "type": "string",
            "title": "Name",
        }
