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
