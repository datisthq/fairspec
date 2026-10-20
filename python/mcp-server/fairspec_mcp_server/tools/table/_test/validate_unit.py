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

    def test_should_return_at_most_100_errors_by_default(self):
        resource = Resource(
            data=[{"age": "invalid"}] * 300,
            tableSchema=TableSchema.model_validate(
                {"properties": {"age": {"type": "integer"}}}
            ),
        )

        result = validate_table_tool(resource)

        assert result.report.valid is False
        assert len(result.report.errors) == 100
        assert result.truncated is True

    def test_should_respect_max_errors(self):
        resource = Resource(
            data=[{"age": "invalid"}] * 10,
            tableSchema=TableSchema.model_validate(
                {"properties": {"age": {"type": "integer"}}}
            ),
        )

        result = validate_table_tool(resource, maxErrors=3)

        assert len(result.report.errors) == 3
        assert result.truncated is True

    def test_should_not_mark_complete_report_as_truncated(self):
        path = write_temp_file("id,age\n1,200\n", format="csv")
        schema = TableSchema.model_validate(
            {"properties": {"age": {"type": "integer", "maximum": 150}}}
        )

        result = validate_table_tool(Resource(data=path, tableSchema=schema))

        assert len(result.report.errors) == 1
        assert result.truncated is False

    def test_should_fill_max_errors_from_one_bad_column_of_many(self):
        resource = Resource(
            data=[{"id": i, "age": "invalid"} for i in range(300)],
            tableSchema=TableSchema.model_validate(
                {"properties": {"id": {"type": "integer"}, "age": {"type": "integer"}}}
            ),
        )

        result = validate_table_tool(resource)

        assert len(result.report.errors) == 100
        assert result.truncated is True
