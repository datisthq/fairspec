from __future__ import annotations

from fairspec_metadata import Resource

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

    def test_should_return_valid_for_no_data(self):
        resource = Resource(
            dataSchema={"type": "object"},
        )
        report = validate_data(resource)
        assert report.valid is True
