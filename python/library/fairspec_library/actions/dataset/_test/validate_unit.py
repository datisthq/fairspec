from __future__ import annotations

from fairspec_metadata import (
    Dataset,
    IntegerColumnProperty,
    Resource,
    StringColumnProperty,
    TableSchema,
)

from ..validate import validate_dataset


class TestValidateDataset:
    def test_should_validate_valid_dataset(self):
        dataset = Dataset(
            resources=[
                Resource(
                    data=[{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}],
                    name="test",
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "name": StringColumnProperty(),
                        }
                    ),
                )
            ]
        )
        report = validate_dataset(dataset)
        assert report.valid is True

    def test_should_detect_invalid_resource(self):
        dataset = Dataset(
            resources=[
                Resource(
                    data=[{"id": "BAD", "name": "english"}],
                    name="test",
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "name": StringColumnProperty(),
                        }
                    ),
                )
            ]
        )
        report = validate_dataset(dataset)
        assert report.valid is False

    def test_should_handle_empty_dataset(self):
        dataset = Dataset(resources=[])
        report = validate_dataset(dataset)
        assert report.valid is True

    def test_should_return_invalid_report_for_bad_descriptor(self):
        report = validate_dataset({"resources": "bad"})
        assert report.valid is False
        assert report.errors

    def test_should_validate_valid_descriptor(self):
        descriptor = {
            "resources": [
                {
                    "name": "test",
                    "data": [{"id": 1, "name": "english"}],
                    "tableSchema": {
                        "properties": {
                            "id": {"type": "integer"},
                            "name": {"type": "string"},
                        }
                    },
                }
            ]
        }
        report = validate_dataset(descriptor)
        assert report.valid is True


class TestValidateDatasetConcurrency:
    def _create_dataset(self) -> Dataset:
        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            }
        )
        return Dataset(
            resources=[
                Resource(
                    data=[{"id": "BAD", "name": "english"}, {"id": 2, "name": "中文"}],
                    tableSchema=table_schema,
                ),
                Resource(
                    data=[{"id": 1, "name": "english"}],
                    tableSchema=table_schema,
                ),
                Resource(
                    data=[{"id": "ALSO BAD", "name": "français"}],
                    tableSchema=table_schema,
                ),
            ]
        )

    def test_should_produce_identical_report_for_serial_and_concurrent(self):
        serial = validate_dataset(self._create_dataset(), concurrency=1)
        concurrent = validate_dataset(self._create_dataset(), concurrency=4)

        assert serial.model_dump() == concurrent.model_dump()

    def test_should_assign_resource_names_before_validating(self):
        report = validate_dataset(self._create_dataset(), concurrency=4)

        assert report.valid is False
        assert [error.resourceName for error in report.errors] == [
            "resource1",
            "resource3",
        ]
