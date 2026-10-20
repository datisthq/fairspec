from fairspec_library import Resource

from ..validate import validate_data_tool


class TestValidateDataTool:
    def test_should_validate_data_against_schema(self):
        resource = Resource(
            data={"name": "alice"},
            dataSchema={"type": "object", "properties": {"name": {"type": "string"}}},
        )

        result = validate_data_tool(resource)

        assert result.report.valid is True

    def test_should_detect_invalid_data(self):
        resource = Resource(
            data={"name": 1},
            dataSchema={"type": "object", "properties": {"name": {"type": "string"}}},
        )

        result = validate_data_tool(resource)

        assert result.report.valid is False
        assert len(result.report.errors) > 0
