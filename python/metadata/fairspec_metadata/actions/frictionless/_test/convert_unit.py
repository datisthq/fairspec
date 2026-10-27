from ..convert import convert_descriptor_from_frictionless, get_is_frictionless_descriptor


class TestGetIsFrictionlessDescriptor:
    def test_recognises_a_schema_by_its_fields(self):
        assert get_is_frictionless_descriptor({"fields": []}, kind="tableSchema")
        assert not get_is_frictionless_descriptor({"properties": {}}, kind="tableSchema")

    def test_recognises_a_dialect_without_a_format(self):
        assert get_is_frictionless_descriptor({"delimiter": ";"}, kind="fileDialect")
        assert not get_is_frictionless_descriptor({"format": "csv"}, kind="fileDialect")

    def test_recognises_a_package_by_its_resources_or_profile(self):
        def is_package(descriptor: dict) -> bool:
            return get_is_frictionless_descriptor(descriptor, kind="dataset")

        assert is_package({"resources": [{"path": "a.csv"}]})
        assert is_package({"profile": "data-package", "resources": []})
        assert not is_package({"resources": [{"data": "a.csv"}]})

    def test_never_takes_a_fairspec_descriptor_for_a_frictionless_one(self):
        descriptor = {
            "$schema": "https://fairspec.org/profiles/latest/table-schema.json",
            "fields": [],
        }
        assert not get_is_frictionless_descriptor(descriptor, kind="tableSchema")


class TestConvertDescriptorFromFrictionless:
    def test_reports_a_descriptor_the_model_cannot_read(self):
        result = convert_descriptor_from_frictionless(
            {"fields": [{"name": 1}]}, kind="tableSchema"
        )

        assert result.descriptor is None
        assert [error.jsonPointer for error in result.errors] == ["/fields/0/name"]

    def test_infers_the_format_of_a_standalone_dialect(self):
        def convert(descriptor: dict) -> object:
            result = convert_descriptor_from_frictionless(descriptor, kind="fileDialect")
            assert result.descriptor is not None
            return result.descriptor["format"]

        assert convert({"delimiter": ";"}) == "csv"
        assert convert({"property": "rows"}) == "json"
        assert convert({"sheetName": "Data"}) == "xlsx"
        assert convert({"table": "items"}) == "sqlite"
