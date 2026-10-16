from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import (
    IntegerColumnProperty,
    Resource,
    StringColumnProperty,
    TableSchema,
)

from ..validate import validate_table


class TestValidateTable:
    def test_should_validate_valid_table(self):
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
        report = validate_table(resource)
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
        report = validate_table(resource)
        assert report.valid is False
        assert len(report.errors) > 0

    def test_should_validate_without_schema(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        resource = Resource(data=path)
        report = validate_table(resource)
        assert report.valid is True
