import json
import os

from fairspec_dataset import get_temp_folder_path
from fairspec_metadata import Dataset, Resource

from ..load import load_dataset_from_frictionless
from ..save import save_dataset_to_frictionless


class TestSaveDatasetToFrictionless:
    def test_writes_paths_relative_to_the_descriptor(self):
        folder = get_temp_folder_path()
        os.makedirs(folder, exist_ok=True)
        dataset = Dataset(
            resources=[Resource(name="items", data=os.path.join(folder, "items.csv"))]
        )

        target = os.path.join(folder, "datapackage.json")
        save_dataset_to_frictionless(dataset, target=target)

        descriptor = read_json(target)
        assert descriptor["resources"] == [{"name": "items", "path": "items.csv"}]

    def test_round_trips_a_package(self):
        folder = get_temp_folder_path()
        os.makedirs(folder, exist_ok=True)
        original = {
            "$schema": "https://datapackage.org/profiles/2.0/datapackage.json",
            "title": "Fruit",
            "licenses": [{"name": "CC0-1.0", "title": "CC0-1.0"}],
            "contributors": [{"title": "Ann", "roles": ["author"]}],
            "resources": [
                {
                    "name": "fruit",
                    "path": "fruit.csv",
                    "type": "table",
                    "format": "csv",
                    "dialect": {"delimiter": ";"},
                    "schema": {
                        "fields": [
                            {
                                "name": "id",
                                "type": "integer",
                                "constraints": {"required": True},
                            },
                            {"name": "price", "type": "number", "decimalChar": ","},
                        ],
                        "primaryKey": ["id"],
                    },
                }
            ],
        }
        path = os.path.join(folder, "datapackage.json")
        with open(path, "w", encoding="utf-8") as file:
            json.dump(original, file)

        dataset = Dataset.model_validate(load_dataset_from_frictionless(path))
        save_dataset_to_frictionless(dataset, target=path, overwrite=True)

        assert read_json(path) == original


def read_json(path: str) -> dict:
    with open(path, encoding="utf-8") as file:
        return json.load(file)
