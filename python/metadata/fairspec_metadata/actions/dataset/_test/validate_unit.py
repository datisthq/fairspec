from ..validate import validate_dataset_descriptor


class TestValidateDatasetDescriptor:
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
