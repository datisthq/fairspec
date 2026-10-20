from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import (
    IntegerColumnProperty,
    Resource,
    RowCheckError,
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

    def test_should_report_rows_failing_a_check(self):
        path = write_temp_file("id,start,end\n1,1,2\n2,3,2", format="csv")
        resource = Resource(data=path)

        report = validate_table(
            resource,
            checks=[{"name": "end-after-start", "condition": '"end" >= start'}],
        )

        assert report.valid is False
        assert report.errors == [
            RowCheckError(type="row/check", checkName="end-after-start", rowNumber=2)
        ]

    def test_should_run_checks_when_schema_inference_is_skipped(self):
        path = write_temp_file("id,start,end\n1,1,2\n2,3,2", format="csv")
        resource = Resource(data=path)

        report = validate_table(
            resource,
            noInfer=True,
            checks=[{"name": "two-rows", "condition": "count(*) = 2"}],
        )

        assert report.valid is True
