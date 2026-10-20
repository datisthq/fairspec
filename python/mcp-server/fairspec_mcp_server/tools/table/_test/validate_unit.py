from fairspec_library import Resource, TableSchema, write_temp_file

from ..validate import validate_table_tool


class TestValidateTableTool:
    def test_should_validate_valid_table(self):
        path = write_temp_file("id,age\n1,25\n2,30\n", format="csv")

        result = validate_table_tool(Resource(data=path))

        assert result.report.valid is True

    def test_should_detect_invalid_table(self):
        path = write_temp_file("id,age\n1,200\n2,300\n", format="csv")
        schema = TableSchema.model_validate(
            {"properties": {"age": {"type": "integer", "maximum": 150}}}
        )

        result = validate_table_tool(Resource(data=path, tableSchema=schema))

        assert result.report.valid is False
        assert len(result.report.errors) == 2

    def test_should_forward_options(self):
        path = write_temp_file("id,age\n1,200\n2,300\n", format="csv")
        schema = TableSchema.model_validate(
            {"properties": {"age": {"type": "integer", "maximum": 150}}}
        )

        result = validate_table_tool(
            Resource(data=path, tableSchema=schema), {"maxErrors": 1}
        )

        assert len(result.report.errors) == 1
