from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import (
    Dataset,
    ForeignKey,
    ForeignKeyError,
    ForeignKeyReference,
    IntegerColumnProperty,
    Resource,
    ResourceMissingError,
    StringColumnProperty,
    TableSchema,
)

from ..foreign_key import validate_dataset_foreign_keys


class TestValidateDatasetForeignKeys:
    def test_should_validate_valid_foreign_keys(self):
        path1 = write_temp_file("id,name\n1,english\n2,中文", format="csv")
        path2 = write_temp_file("id,name_id\n1,1\n2,2", format="csv")
        dataset = Dataset(
            resources=[
                Resource(
                    data=path1,
                    name="names",
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "name": StringColumnProperty(),
                        }
                    ),
                ),
                Resource(
                    data=path2,
                    name="refs",
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "name_id": IntegerColumnProperty(),
                        },
                        foreignKeys=[
                            ForeignKey(
                                columns=["name_id"],
                                reference=ForeignKeyReference(
                                    resource="names",
                                    columns=["id"],
                                ),
                            )
                        ],
                    ),
                ),
            ]
        )
        report = validate_dataset_foreign_keys(dataset)
        assert report.valid is True

    def test_should_report_missing_referenced_resource(self):
        path = write_temp_file("id,name_id\n1,1", format="csv")
        dataset = Dataset(
            resources=[
                Resource(
                    data=path,
                    name="refs",
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "name_id": IntegerColumnProperty(),
                        },
                        foreignKeys=[
                            ForeignKey(
                                columns=["name_id"],
                                reference=ForeignKeyReference(
                                    resource="names", columns=["id"]
                                ),
                            )
                        ],
                    ),
                )
            ]
        )
        report = validate_dataset_foreign_keys(dataset)
        assert len(report.errors) == 1
        error = report.errors[0]
        assert isinstance(error, ResourceMissingError)
        assert error.resourceName == "names"
        assert error.referencingResourceName == "refs"

    def test_should_check_self_references_and_skip_nulls(self):
        path = write_temp_file("id,parent_id\n1,\n2,1\n3,9", format="csv")
        dataset = Dataset(
            resources=[
                Resource(
                    data=path,
                    name="tree",
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "parent_id": IntegerColumnProperty(),
                        },
                        foreignKeys=[
                            ForeignKey(
                                columns=["parent_id"],
                                reference=ForeignKeyReference(columns=["id"]),
                            )
                        ],
                    ),
                )
            ]
        )
        report = validate_dataset_foreign_keys(dataset)
        assert len(report.errors) == 1
        error = report.errors[0]
        assert isinstance(error, ForeignKeyError)
        assert error.cells == ["9"]

    def test_should_detect_foreign_key_violation(self):
        path1 = write_temp_file("id,name\n1,english\n2,中文", format="csv")
        path2 = write_temp_file("id,name_id\n1,1\n2,999", format="csv")
        dataset = Dataset(
            resources=[
                Resource(
                    data=path1,
                    name="names",
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "name": StringColumnProperty(),
                        }
                    ),
                ),
                Resource(
                    data=path2,
                    name="refs",
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "name_id": IntegerColumnProperty(),
                        },
                        foreignKeys=[
                            ForeignKey(
                                columns=["name_id"],
                                reference=ForeignKeyReference(
                                    resource="names",
                                    columns=["id"],
                                ),
                            )
                        ],
                    ),
                ),
            ]
        )
        report = validate_dataset_foreign_keys(dataset)
        assert report.valid is False
        assert len(report.errors) > 0

    def test_should_handle_no_foreign_keys(self):
        path = write_temp_file("id,name\n1,english", format="csv")
        dataset = Dataset(
            resources=[
                Resource(
                    data=path,
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
        report = validate_dataset_foreign_keys(dataset)
        assert report.valid is True
