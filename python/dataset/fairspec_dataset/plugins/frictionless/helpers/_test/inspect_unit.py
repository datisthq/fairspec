import pytest

from fairspec_metadata import FairspecException, MetadataError

from ...models.package import FrictionlessPackage
from ..inspect import assert_frictionless_model


def get_pointers(descriptor: dict) -> list[str]:
    with pytest.raises(FairspecException) as info:
        assert_frictionless_model(descriptor, model=FrictionlessPackage)
    assert info.value.report is not None
    errors = info.value.report.errors
    assert all(isinstance(error, MetadataError) for error in errors)
    return [error.jsonPointer for error in errors if isinstance(error, MetadataError)]


class TestAssertFrictionlessModel:
    def test_parses_a_package_with_extra_properties(self):
        package = assert_frictionless_model(
            {"resources": [{"name": "a", "path": "a.csv", "custom": 1}]},
            model=FrictionlessPackage,
        )
        assert package.resources[0].path == "a.csv"

    def test_reports_each_mismatch_by_pointer(self):
        descriptor = {"resources": [{"bytes": "many", "licenses": [{"name": 1}]}]}
        assert get_pointers(descriptor) == [
            "/resources/0/bytes",
            "/resources/0/licenses/0/name",
        ]

    def test_reports_missing_resources(self):
        assert get_pointers({}) == ["/resources"]

    def test_does_not_coerce_strings_and_numbers(self):
        descriptor = {"resources": [{"bytes": "5", "schema": {"fields": [{"name": 1}]}}]}
        assert get_pointers(descriptor) == ["/resources/0/bytes", "/resources/0/schema"]

    def test_rejects_null_for_an_optional_property(self):
        assert get_pointers({"title": None, "resources": []}) == ["/title"]

    def test_prefixes_the_given_pointer(self):
        with pytest.raises(FairspecException) as info:
            assert_frictionless_model(
                {}, model=FrictionlessPackage, json_pointer="/resources/0/schema"
            )
        assert info.value.report is not None
        error = info.value.report.errors[0]
        assert isinstance(error, MetadataError)
        assert error.jsonPointer == "/resources/0/schema/resources"
