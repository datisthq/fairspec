import json

from fairspec_library import Dataset, Resource, TableSchema, write_temp_file

from ..validate import validate_dataset_tool


class TestValidateDatasetTool:
    def test_should_validate_valid_dataset(self):
        dataset = Dataset(
            resources=[
                Resource(
                    name="people",
                    data=[{"id": 1}, {"id": 2}],
                    tableSchema=TableSchema.model_validate(
                        {"properties": {"id": {"type": "integer"}}}
                    ),
                )
            ]
        )

        result = validate_dataset_tool(dataset)

        assert result.report.valid is True
        assert result.report.errors == []

    def test_should_detect_invalid_resource_data(self):
        dataset = Dataset(
            resources=[
                Resource(
                    name="people",
                    data=[{"id": "one"}],
                    tableSchema=TableSchema.model_validate(
                        {"properties": {"id": {"type": "integer"}}}
                    ),
                )
            ]
        )

        result = validate_dataset_tool(dataset)

        assert result.report.valid is False
        assert len(result.report.errors) > 0

    def test_should_validate_dataset_from_path(self):
        path = write_temp_file(
            json.dumps({"resources": [{"name": "people", "data": [{"id": 1}]}]}),
            format="json",
        )

        result = validate_dataset_tool(path)

        assert result.report.valid is True
