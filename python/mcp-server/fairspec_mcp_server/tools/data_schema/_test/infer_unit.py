from fairspec_library import Resource

from ..infer import infer_data_schema_tool


class TestInferDataSchemaTool:
    def test_should_infer_schema_from_inline_data(self):
        resource = Resource(data={"name": "alice", "age": 25})

        result = infer_data_schema_tool(resource)

        assert result.dataSchema is not None
        assert result.dataSchema["type"] == "object"
        assert set(result.dataSchema["properties"]) == {"name", "age"}

    def test_should_return_none_for_resource_without_data(self):
        result = infer_data_schema_tool(Resource())

        assert result.dataSchema is None
        assert result.model_dump(mode="json", by_alias=True) == {}
