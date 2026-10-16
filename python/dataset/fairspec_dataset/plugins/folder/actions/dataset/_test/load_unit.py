import os

import pytest
from fairspec_metadata import DescriptionType
from fairspec_metadata import DataciteDescription
from fairspec_metadata import Title
from fairspec_metadata import Dataset
from fairspec_metadata import CsvFileDialect
from fairspec_metadata import Resource
from fairspec_metadata import IntegerColumnProperty
from fairspec_metadata import StringColumnProperty
from fairspec_metadata import TableSchema

from fairspec_dataset.actions.file.temp import get_temp_file_path, write_temp_file
from fairspec_dataset.actions.folder.temp import get_temp_folder_path
from ..load import load_dataset_from_folder
from ..save import save_dataset_to_folder


class TestLoadDatasetFromFolder:
    def test_loads_basic_dataset_from_folder(self):
        folder = get_temp_file_path()
        dataset = Dataset(resources=[Resource(name="test_res", data=[{"id": 1}])])
        save_dataset_to_folder(dataset, folder_path=folder)

        result = load_dataset_from_folder(folder)

        assert result.resources is not None
        assert len(result.resources) == 1

    def test_loads_dataset_with_metadata(self):
        folder = get_temp_file_path()
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
        save_dataset_to_folder(dataset, folder_path=folder)

        result = load_dataset_from_folder(folder)

        assert result.titles is not None
        assert result.titles[0].title == "Test Dataset"
        assert result.descriptions is not None
        assert result.descriptions[0].description == "A test"
        assert result.version == "1.0"

    def test_loads_dataset_with_inline_data_resources(self):
        folder = get_temp_file_path()
        data = [{"id": 1, "name": "alice"}, {"id": 2, "name": "bob"}]
        dataset = Dataset(resources=[Resource(name="test_res", data=data)])
        save_dataset_to_folder(dataset, folder_path=folder)

        result = load_dataset_from_folder(folder)

        assert result.resources is not None
        assert result.resources[0].data == data

    def test_loads_dataset_with_file_resources(self):
        csv_path = write_temp_file("id,name\n1,alice\n2,bob\n", format="csv")
        folder = get_temp_file_path()
        dataset = Dataset(
            resources=[
                Resource(
                    name="test_res",
                    data=csv_path,
                    fileDialect=CsvFileDialect(),
                )
            ]
        )
        save_dataset_to_folder(dataset, folder_path=folder)

        result = load_dataset_from_folder(folder)

        assert result.resources is not None
        assert result.resources[0].fileDialect is not None

    def test_loads_dataset_with_table_schema(self):
        folder = get_temp_file_path()
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
        save_dataset_to_folder(dataset, folder_path=folder)

        result = load_dataset_from_folder(folder)

        assert result.resources is not None
        assert isinstance(result.resources[0].tableSchema, TableSchema)
        assert result.resources[0].tableSchema.properties is not None
        assert len(result.resources[0].tableSchema.properties) == 2

    def test_loads_dataset_with_multiple_resources(self):
        csv_path = write_temp_file("id,name\n1,alice\n", format="csv")
        folder = get_temp_file_path()
        dataset = Dataset(
            resources=[
                Resource(name="file_res", data=csv_path),
                Resource(name="inline_res", data=[{"id": 1}]),
            ]
        )
        save_dataset_to_folder(dataset, folder_path=folder)

        result = load_dataset_from_folder(folder)

        assert result.resources is not None
        assert len(result.resources) == 2
        assert result.resources[0].name == "file_res"
        assert result.resources[1].name == "inline_res"

    def test_loads_dataset_with_delimiter(self):
        csv_path = write_temp_file("id;name\n1;alice\n", format="csv")
        folder = get_temp_file_path()
        dataset = Dataset(
            resources=[
                Resource(
                    name="test_res",
                    data=csv_path,
                    fileDialect=CsvFileDialect(delimiter=";"),
                )
            ]
        )
        save_dataset_to_folder(dataset, folder_path=folder)

        result = load_dataset_from_folder(folder)

        assert result.resources is not None
        assert isinstance(result.resources[0].fileDialect, CsvFileDialect)
        assert result.resources[0].fileDialect.delimiter == ";"

    def test_throws_error_for_non_existent_folder(self):
        with pytest.raises(Exception):
            load_dataset_from_folder("/non/existent/folder")

    def test_throws_error_for_folder_without_dataset_json(self):
        folder = get_temp_folder_path()
        os.makedirs(folder, exist_ok=True)
        with pytest.raises(Exception):
            load_dataset_from_folder(folder)
