from ....models.frictionless.package import FrictionlessPackage
from ..inspect import inspect_frictionless_model


def get_pointers(descriptor: dict) -> list[str]:
    package, errors = inspect_frictionless_model(descriptor, model=FrictionlessPackage)
    assert package is None
    return [error.jsonPointer for error in errors]


class TestInspectFrictionlessModel:
    def test_parses_a_package_with_extra_properties(self):
        package, errors = inspect_frictionless_model(
            {"resources": [{"name": "a", "path": "a.csv", "custom": 1}]},
            model=FrictionlessPackage,
        )
        assert errors == []
        assert package is not None
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
        _, errors = inspect_frictionless_model(
            {}, model=FrictionlessPackage, root_json_pointer="/resources/0/schema"
        )
        assert errors[0].jsonPointer == "/resources/0/schema/resources"
