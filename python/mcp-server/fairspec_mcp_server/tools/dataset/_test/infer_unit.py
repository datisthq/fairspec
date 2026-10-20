from fairspec_library import Dataset, Resource, write_temp_file

from ..infer import infer_dataset_tool


class TestInferDatasetTool:
    def test_should_infer_resource_metadata(self):
        path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")
        dataset = Dataset(resources=[Resource(data=path)])

        result = infer_dataset_tool(dataset)

        assert result.dataset.resources
        resource = result.dataset.resources[0]
        assert resource.fileDialect is not None
        assert resource.tableSchema is not None

    def test_should_preserve_dataset_metadata(self):
        dataset = Dataset(version="1.0.0", resources=[])

        result = infer_dataset_tool(dataset)

        assert result.dataset.version == "1.0.0"
