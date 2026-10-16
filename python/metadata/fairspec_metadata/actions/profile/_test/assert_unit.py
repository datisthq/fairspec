import pytest

from fairspec_metadata.models.profile import ProfileType

from ..assert_ import Error, assert_profile


class TestAssertProfileValid:
    def test_valid_dataset_profile(self):
        schema = {"type": "object"}
        result = assert_profile(
            schema,
            path="https://fairspec.org/profiles/latest/dataset.json",
            type=ProfileType.dataset,
        )
        assert result == schema

    def test_valid_versioned_profile(self):
        schema = {"type": "object"}
        result = assert_profile(
            schema,
            path="https://fairspec.org/profiles/1.0.0/catalog.json",
            type=ProfileType.catalog,
        )
        assert result == schema

    def test_valid_profile_via_allof(self):
        schema = {
            "type": "object",
            "allOf": ["https://fairspec.org/profiles/latest/table-schema.json"],
        }
        result = assert_profile(
            schema,
            path="https://example.com/custom.json",
            type=ProfileType.table_schema,
        )
        assert result == schema

    def test_data_schema_accepts_json_schema_draft(self):
        schema = {"type": "object"}
        result = assert_profile(
            schema,
            path="https://json-schema.org/draft/2020-12/schema",
            type=ProfileType.data_schema,
        )
        assert result == schema

    def test_valid_file_dialect_profile(self):
        schema = {"type": "object"}
        result = assert_profile(
            schema,
            path="https://fairspec.org/profiles/latest/file-dialect.json",
            type=ProfileType.file_dialect,
        )
        assert result == schema


class TestAssertProfileInvalid:
    def test_wrong_type_in_path(self):
        with pytest.raises(Error, match="not a valid dataset profile"):
            assert_profile(
                {"type": "object"},
                path="https://fairspec.org/profiles/latest/catalog.json",
                type=ProfileType.dataset,
            )

    def test_wrong_domain(self):
        with pytest.raises(Error):
            assert_profile(
                {"type": "object"},
                path="https://other.org/profiles/latest/dataset.json",
                type=ProfileType.dataset,
            )

    def test_invalid_version_format(self):
        with pytest.raises(Error):
            assert_profile(
                {"type": "object"},
                path="https://fairspec.org/profiles/v1/dataset.json",
                type=ProfileType.dataset,
            )

    def test_json_schema_draft_only_for_data_schema(self):
        with pytest.raises(Error):
            assert_profile(
                {"type": "object"},
                path="https://json-schema.org/draft/2020-12/schema",
                type=ProfileType.dataset,
            )
