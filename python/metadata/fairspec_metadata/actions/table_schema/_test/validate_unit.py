import pytest

from ..validate import validate_table_schema


class TestValidateTableSchema:
    def test_valid_schema(self):
        descriptor = {
            "properties": {
                "id": {"type": "integer"},
                "name": {"type": "string"},
            },
        }
        result = validate_table_schema(descriptor)
        assert result.valid is True
        assert result.errors == []

    def test_invalid_schema(self):
        descriptor = {
            "properties": {
                "id": {"type": 123},
            },
        }
        result = validate_table_schema(descriptor)
        assert result.valid is False
        assert len(result.errors) > 0

    def test_missing_schema_is_valid(self):
        descriptor = {
            "properties": {
                "id": {"type": "integer"},
            },
        }
        result = validate_table_schema(descriptor)
        assert result.valid is True

    def test_wrong_profile_type_raises(self):
        descriptor = {
            "$schema": "https://fairspec.org/profiles/latest/dataset.json",
            "properties": {
                "id": {"type": "integer"},
            },
        }
        with pytest.raises(Exception, match="table-schema"):
            validate_table_schema(descriptor)
