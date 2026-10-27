from fairspec_metadata.models.resource import Resource

from ..support import get_supported_file_dialect


class TestGetSupportedFileDialect:
    def test_declared_builtin_format(self):
        resource = Resource.model_validate(
            {"data": "table.txt", "fileDialect": {"format": "csv"}}
        )
        file_dialect = get_supported_file_dialect(resource, ["csv"])
        assert file_dialect is not None
        assert file_dialect.format == "csv"

    def test_inferred_format(self):
        resource = Resource.model_validate({"data": "table.csv"})
        file_dialect = get_supported_file_dialect(resource, ["csv"])
        assert file_dialect is not None
        assert file_dialect.format == "csv"

    def test_custom_format_is_not_inferred_from_extension(self):
        resource = Resource.model_validate(
            {"data": "table.csv", "fileDialect": {"format": "custom"}}
        )
        assert get_supported_file_dialect(resource, ["csv"]) is None
