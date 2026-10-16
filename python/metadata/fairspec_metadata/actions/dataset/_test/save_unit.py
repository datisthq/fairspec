import json
from unittest.mock import patch

import pytest

from fairspec_metadata.models.datacite.creator import Creator
from fairspec_metadata.models.datacite.title import Title
from fairspec_metadata.actions.dataset.validate import validate_dataset_descriptor
from fairspec_metadata.actions.json_schema.load import load_json_schema
from fairspec_metadata.models.dataset import Dataset
from fairspec_metadata.models.integrity import Integrity, IntegrityType
from fairspec_metadata.models.resource import Resource
from fairspec_metadata.settings import FAIRSPEC_VERSION

from ..save import save_dataset_descriptor


class TestSaveDatasetDescriptor:
    def test_saves_dataset(self, tmp_path):
        path = str(tmp_path / "dataset.json")
        dataset = Dataset(
            creators=[Creator(name="Test Creator")],
            titles=[Title(title="Test Dataset")],
            resources=[
                Resource(name="test_resource", data=str(tmp_path / "data.csv")),
            ],
        )
        save_dataset_descriptor(dataset, path=path)
        with open(path, encoding="utf-8") as f:
            content = json.load(f)
        assert content["$schema"].endswith("dataset.json")
        assert content["creators"][0]["name"] == "Test Creator"
        assert content["resources"][0]["name"] == "test_resource"

    def test_sets_default_schema(self, tmp_path):
        path = str(tmp_path / "dataset.json")
        dataset = Dataset(
            resources=[
                Resource(data=str(tmp_path / "data.csv")),
            ],
        )
        save_dataset_descriptor(dataset, path=path)
        with open(path, encoding="utf-8") as f:
            content = json.load(f)
        expected = f"https://fairspec.org/profiles/{FAIRSPEC_VERSION}/dataset.json"
        assert content["$schema"] == expected

    def test_preserves_custom_schema(self, tmp_path):
        path = str(tmp_path / "dataset.json")
        dataset = Dataset(
            profile="https://custom.schema.url/dataset.json",
            resources=[
                Resource(data=str(tmp_path / "data.csv")),
            ],
        )
        save_dataset_descriptor(dataset, path=path)
        with open(path, encoding="utf-8") as f:
            content = json.load(f)
        assert content["$schema"] == "https://custom.schema.url/dataset.json"

    def test_throws_when_file_exists(self, tmp_path):
        path = str(tmp_path / "dataset.json")
        dataset = Dataset(resources=[Resource(data=str(tmp_path / "data.csv"))])
        save_dataset_descriptor(dataset, path=path)
        with pytest.raises(FileExistsError):
            save_dataset_descriptor(dataset, path=path)

    def test_overwrites_when_flag_set(self, tmp_path):
        path = str(tmp_path / "dataset.json")
        dataset1 = Dataset(
            creators=[Creator(name="Initial")],
            resources=[Resource(data=str(tmp_path / "data.csv"))],
        )
        dataset2 = Dataset(
            creators=[Creator(name="Updated")],
            resources=[Resource(data=str(tmp_path / "data.csv"))],
        )
        save_dataset_descriptor(dataset1, path=path)
        save_dataset_descriptor(dataset2, path=path, overwrite=True)
        with open(path, encoding="utf-8") as f:
            content = json.load(f)
        assert content["creators"][0]["name"] == "Updated"

    def test_saves_to_nested_directory(self, tmp_path):
        path = str(tmp_path / "nested" / "dir" / "dataset.json")
        dataset = Dataset(
            resources=[
                Resource(data=str(tmp_path / "nested" / "dir" / "data.csv")),
            ],
        )
        save_dataset_descriptor(dataset, path=path)
        with open(path, encoding="utf-8") as f:
            content = json.load(f)
        assert "resources" in content

    def test_denormalizes_resource_paths(self, tmp_path):
        path = str(tmp_path / "dataset.json")
        dataset = Dataset(
            resources=[
                Resource(name="test", data=str(tmp_path / "data.csv")),
            ],
        )
        save_dataset_descriptor(dataset, path=path)
        with open(path, encoding="utf-8") as f:
            content = json.load(f)
        assert content["resources"][0]["data"] == "data.csv"

    def test_saved_dataset_validates_offline(self, tmp_path):
        path = str(tmp_path / "dataset.json")
        dataset = Dataset(
            resources=[
                Resource(
                    name="test_resource",
                    data=str(tmp_path / "data.csv"),
                    integrity=Integrity(type=IntegrityType.sha256, hash="abc"),
                ),
            ],
        )
        save_dataset_descriptor(dataset, path=path)

        load_json_schema.cache_clear()
        with patch(
            "fairspec_metadata.actions.descriptor.load.urllib.request.urlopen",
            side_effect=OSError("offline"),
        ):
            report = validate_dataset_descriptor(path)

        assert report.errors == []
        assert report.valid
