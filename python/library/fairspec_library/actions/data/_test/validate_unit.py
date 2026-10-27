from __future__ import annotations

from fairspec_metadata import Resource, ResourceTypeError

from ..validate import validate_data


class TestValidateData:
    def test_should_return_valid_when_no_schema(self):
        resource = Resource(data=[{"id": 1}])
        report = validate_data(resource)
        assert report.valid is True

    def test_should_validate_inline_data(self):
        resource = Resource(
            data={"name": "test", "age": 25},
            dataSchema={
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "age": {"type": "integer"},
                },
                "required": ["name", "age"],
            },
        )
        report = validate_data(resource)
        assert report.valid is True

    def test_should_detect_invalid_data(self):
        resource = Resource(
            data={"name": 123},
            dataSchema={
                "type": "object",
                "properties": {"name": {"type": "string"}},
            },
        )
        report = validate_data(resource)
        assert report.valid is False
        assert len(report.errors) > 0

    def test_should_return_error_when_no_data_but_schema_expected(self):
        resource = Resource(
            dataSchema={"type": "object"},
        )
        report = validate_data(resource)
        assert report.valid is False
        assert len(report.errors) == 1
        error = report.errors[0]
        assert isinstance(error, ResourceTypeError)
        assert error.expectedResourceType == "data"

    def test_should_validate_empty_inline_object(self):
        resource = Resource(
            data={},
            dataSchema={"type": "object", "required": ["name"]},
        )
        report = validate_data(resource)
        assert report.valid is False
        assert len(report.errors) == 1

    def test_should_assert_a_format_it_knows(self):
        report = validate_data(
            Resource(
                data={"day": "2023-02-29"},
                dataSchema={"properties": {"day": {"type": "string", "format": "date"}}},
            )
        )
        assert [error.type for error in report.errors] == ["data"]
        assert report.errors[0].jsonPointer == "/day"  # type: ignore[union-attr]

    def test_should_leave_an_unknown_format_an_annotation(self):
        report = validate_data(
            Resource(
                data={"site": "not a uri"},
                dataSchema={"properties": {"site": {"type": "string", "format": "uri"}}},
            )
        )
        assert report.valid is True

    def test_should_report_a_remote_ref_as_metadata_without_fetching_it(self):
        report = validate_data(
            Resource(
                data={"a": 1},
                dataSchema={
                    "properties": {"a": {"$ref": "https://example.com/schema.json"}}
                },
            )
        )
        assert [error.type for error in report.errors] == ["metadata"]
        assert report.errors[0].jsonPointer == "/dataSchema/properties/a/$ref"  # type: ignore[union-attr]
