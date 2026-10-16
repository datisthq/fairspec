from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import (
    IntegerColumnProperty,
    Resource,
    StringColumnProperty,
    TableSchema,
)

from ..validate import validate_resource


class TestValidateResource:
    def test_should_validate_valid_resource(self):
        path = write_temp_file("id,name\n1,english\n2,中文", format="csv")
        resource = Resource(
            data=path,
            tableSchema=TableSchema(
                properties={
                    "id": IntegerColumnProperty(),
                    "name": StringColumnProperty(),
                }
            ),
        )
        report = validate_resource(resource)
        assert report.valid is True

    def test_should_detect_type_error(self):
        path = write_temp_file("id,name\nBAD,english", format="csv")
        resource = Resource(
            data=path,
            tableSchema=TableSchema(
                properties={
                    "id": IntegerColumnProperty(),
                    "name": StringColumnProperty(),
                }
            ),
        )
        report = validate_resource(resource)
        assert report.valid is False

    def test_should_validate_resource_without_schema(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        resource = Resource(data=path)
        report = validate_resource(resource)
        assert report.valid is True
