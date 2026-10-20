import json

from fairspec_library import Report, write_temp_file

from ..validate import validate_data_schema_tool


class TestValidateDataSchemaTool:
    def test_should_validate_valid_descriptor(self):
        result = validate_data_schema_tool({"type": "object"})

        assert result.report.valid is True
        assert result.report.errors == []

    def test_should_detect_invalid_descriptor(self):
        result = validate_data_schema_tool({"type": 123})

        assert result.report.valid is False
        assert len(result.report.errors) > 0

    def test_should_validate_descriptor_from_path(self):
        path = write_temp_file(json.dumps({"type": "object"}), format="json")

        result = validate_data_schema_tool(path)

        assert result.report.valid is True

    def test_should_return_plain_report(self):
        result = validate_data_schema_tool({"type": "object"})

        assert type(result.report) is Report
