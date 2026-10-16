import os

import pytest
from fairspec_metadata import DescriptionType
from fairspec_metadata import DataciteDescription
from fairspec_metadata import Subject
from fairspec_metadata import Title
from fairspec_metadata import Dataset
from fairspec_metadata import CsvFileDialect
from fairspec_metadata import Resource
from fairspec_metadata import IntegerColumnProperty
from fairspec_metadata import StringColumnProperty
from fairspec_metadata import TableSchema

from fairspec_dataset.actions.file.temp import get_temp_file_path, write_temp_file
from ..load import load_dataset_from_zip
from ..save import save_dataset_to_zip


class TestSaveDatasetToZip:
    def test_saves_basic_dataset_to_zip(self):
        path = get_temp_file_path(format="zip")
        dataset = Dataset(resources=[Resource(name="test_res", data=[{"id": 1}])])

        save_dataset_to_zip(dataset, archive_path=path)

        assert os.path.exists(path)
        assert os.path.getsize(path) > 0

    def test_saves_dataset_with_metadata(self):
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            titles=[Title(title="Test Dataset")],
            descriptions=[
                DataciteDescription(
                    description="A test", descriptionType=DescriptionType.Abstract
                )
            ],
            version="1.0",
            resources=[Resource(name="test_res", data=[{"id": 1}])],
        )

        save_dataset_to_zip(dataset, archive_path=path)

        assert os.path.exists(path)
        assert os.path.getsize(path) > 0

    def test_saves_dataset_with_inline_data_resources(self):
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            resources=[Resource(name="test_res", data=[{"id": 1}, {"id": 2}])]
        )

        save_dataset_to_zip(dataset, archive_path=path)

        assert os.path.exists(path)

    def test_saves_dataset_with_file_resources(self):
        csv_path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")
        path = get_temp_file_path(format="zip")
        dataset = Dataset(resources=[Resource(name="test_res", data=csv_path)])

        save_dataset_to_zip(dataset, archive_path=path)

        assert os.path.exists(path)

    def test_saves_dataset_with_multiple_resources(self):
        csv_path = write_temp_file("id,name\n1,alice\n", format="csv")
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            resources=[
                Resource(name="file_res", data=csv_path),
                Resource(name="inline_res", data=[{"id": 1}]),
            ]
        )

        save_dataset_to_zip(dataset, archive_path=path)

        assert os.path.exists(path)

    def test_saves_dataset_with_table_schema(self):
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            resources=[
                Resource(
                    name="test_res",
                    data=[{"id": 1}],
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "name": StringColumnProperty(),
                        }
                    ),
                )
            ]
        )

        save_dataset_to_zip(dataset, archive_path=path)

        assert os.path.exists(path)

    def test_saves_dataset_with_dialect(self):
        csv_path = write_temp_file("id;name\n1;alice\n", format="csv")
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            resources=[
                Resource(
                    name="test_res",
                    data=csv_path,
                    fileDialect=CsvFileDialect(delimiter=";"),
                )
            ]
        )

        save_dataset_to_zip(dataset, archive_path=path)

        assert os.path.exists(path)

    def test_roundtrip_preserves_structure(self):
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            titles=[Title(title="My Dataset")],
            descriptions=[
                DataciteDescription(
                    description="Desc", descriptionType=DescriptionType.Abstract
                )
            ],
            resources=[Resource(name="test_res", data=[{"id": 1}])],
        )
        save_dataset_to_zip(dataset, archive_path=path)

        result = load_dataset_from_zip(path)

        assert result.titles is not None
        assert result.titles[0].title == "My Dataset"
        assert result.descriptions is not None
        assert result.descriptions[0].description == "Desc"
        assert result.resources is not None
        assert len(result.resources) == 1

    def test_roundtrip_preserves_metadata(self):
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            titles=[Title(title="My Dataset")],
            version="2.0",
            subjects=[Subject(subject="science")],
            resources=[Resource(name="test_res", data=[{"id": 1}])],
        )
        save_dataset_to_zip(dataset, archive_path=path)

        result = load_dataset_from_zip(path)

        assert result.titles is not None
        assert result.titles[0].title == "My Dataset"
        assert result.version == "2.0"
        assert result.subjects is not None
        assert result.subjects[0].subject == "science"

    def test_roundtrip_preserves_table_schema(self):
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            resources=[
                Resource(
                    name="test_res",
                    data=[{"id": 1}],
                    tableSchema=TableSchema(
                        properties={
                            "id": IntegerColumnProperty(),
                            "name": StringColumnProperty(),
                        }
                    ),
                )
            ]
        )
        save_dataset_to_zip(dataset, archive_path=path)

        result = load_dataset_from_zip(path)

        assert result.resources is not None
        assert isinstance(result.resources[0].tableSchema, TableSchema)
        assert result.resources[0].tableSchema.properties is not None
        assert len(result.resources[0].tableSchema.properties) == 2

    def test_roundtrip_preserves_file_resources(self):
        csv_path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            resources=[
                Resource(
                    name="test_res",
                    data=csv_path,
                    fileDialect=CsvFileDialect(),
                )
            ]
        )
        save_dataset_to_zip(dataset, archive_path=path)

        result = load_dataset_from_zip(path)

        assert result.resources is not None
        assert result.resources[0].fileDialect is not None

    def test_throws_error_for_existing_file(self):
        path = write_temp_file("existing content", format="zip")
        dataset = Dataset(resources=[Resource(name="test_res", data=[{"id": 1}])])

        with pytest.raises(FileExistsError):
            save_dataset_to_zip(dataset, archive_path=path)

    def test_creates_valid_zip_structure(self):
        path = get_temp_file_path(format="zip")
        dataset = Dataset(resources=[Resource(name="test_res", data=[{"id": 1}])])
        save_dataset_to_zip(dataset, archive_path=path)

        result = load_dataset_from_zip(path)

        assert result.resources is not None
        assert result.resources[0].name == "test_res"

    def test_roundtrip_with_multiple_file_resources(self):
        csv1 = write_temp_file("id,name\n1,alice\n", filename="data1.csv")
        csv2 = write_temp_file("id,name\n2,bob\n", filename="data2.csv")
        path = get_temp_file_path(format="zip")
        dataset = Dataset(
            resources=[
                Resource(name="first_res", data=csv1),
                Resource(name="second_res", data=csv2),
            ]
        )
        save_dataset_to_zip(dataset, archive_path=path)

        result = load_dataset_from_zip(path)

        assert result.resources is not None
        assert len(result.resources) == 2
        assert result.resources[0].name == "first_res"
        assert result.resources[1].name == "second_res"
