import pytest
from pydantic import TypeAdapter, ValidationError

from fairspec_metadata.models.file_dialect.csv import CsvFileDialect
from fairspec_metadata.models.file_dialect.custom import CustomFileDialect
from fairspec_metadata.models.file_dialect.file_dialect import FileDialect
from fairspec_metadata.models.resource import Resource

from ..validate import validate_file_dialect


class TestValidateFileDialect:
    def test_builtin_format(self):
        result = validate_file_dialect({"format": "csv", "delimiter": ";"})
        assert result.valid
        assert isinstance(result.file_dialect, CsvFileDialect)

    def test_custom_format(self):
        result = validate_file_dialect({"format": "custom", "title": "Custom format"})
        assert result.valid
        assert result.errors == []
        assert isinstance(result.file_dialect, CustomFileDialect)
        assert result.file_dialect.format == "custom"

    def test_missing_format(self):
        result = validate_file_dialect({"title": "Custom format"})
        assert not result.valid


class TestFileDialect:
    def test_custom_format_in_resource(self):
        resource = Resource.model_validate(
            {"data": "file.bin", "fileDialect": {"format": "custom"}}
        )
        assert isinstance(resource.fileDialect, CustomFileDialect)

    def test_builtin_format_with_invalid_property(self):
        with pytest.raises(ValidationError) as error:
            TypeAdapter(FileDialect).validate_python({"format": "csv", "delimiter": 5})
        assert "delimiter" in [item["loc"][-1] for item in error.value.errors()]
