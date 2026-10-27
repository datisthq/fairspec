import os

from ..validate import validate_dataset_descriptor


class TestValidateDatasetDescriptor:
    def test_keeps_extension_properties(self):
        dataset = {"resources": [{"data": "data.csv", "spectralRange": {"min": 400}}]}
        result = validate_dataset_descriptor(dataset)
        assert result.valid
        assert result.dataset is not None
        resources = result.dataset.model_dump(exclude_none=True)["resources"]
        assert resources[0]["spectralRange"] == {"min": 400}

    def test_valid_dataset(self):
        dataset = {"resources": [{"data": "data.csv"}]}
        result = validate_dataset_descriptor(dataset)
        assert result.valid is True
        assert result.errors == []

    def test_invalid_dataset(self):
        dataset = {"resources": "not-an-array"}
        result = validate_dataset_descriptor(dataset)
        assert result.valid is False
        assert len(result.errors) > 0

    def test_missing_schema_is_valid(self):
        dataset = {"resources": [{"data": "data.csv"}]}
        result = validate_dataset_descriptor(dataset)
        assert result.valid is True

    def test_dataset_with_datacite(self):
        dataset = {
            "creators": [{"name": "John Doe"}],
            "titles": [{"title": "Example Dataset"}],
            "resources": [{"data": "data.csv"}],
        }
        result = validate_dataset_descriptor(dataset)
        assert result.valid is True
        assert result.errors == []

    def test_schema_path_outside_working_directory(self, tmp_path, monkeypatch):
        folder = tmp_path / "dataset"
        folder.mkdir()
        (folder / "schema.json").write_text('{"properties": {"id": {"type": "integer"}}}')
        elsewhere = tmp_path / "elsewhere"
        elsewhere.mkdir()
        monkeypatch.chdir(elsewhere)

        dataset = {"resources": [{"data": "data.csv", "tableSchema": "schema.json"}]}
        result = validate_dataset_descriptor(dataset, basepath=str(folder))
        assert result.valid is True
        assert result.dataset is not None
        resource = (result.dataset.resources or [])[0]
        assert isinstance(resource.tableSchema, str)
        assert os.path.samefile(resource.tableSchema, folder / "schema.json")
