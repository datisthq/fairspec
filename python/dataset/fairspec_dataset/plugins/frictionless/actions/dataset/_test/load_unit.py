import json
import os

import pytest

from fairspec_dataset import get_temp_folder_path
from fairspec_metadata import FairspecException

from ..load import load_dataset_from_frictionless


class TestLoadDatasetFromFrictionless:
    def test_loads_a_package_with_schema_and_dialect_paths(self):
        path = write_package(
            {
                "datapackage.json": {
                    "resources": [
                        {
                            "name": "my-items",
                            "path": "items.csv",
                            "schema": "schema.json",
                            "dialect": "dialect.json",
                        }
                    ]
                },
                "schema.json": {"fields": [{"name": "id", "type": "integer"}]},
                "dialect.json": {"delimiter": ";"},
            }
        )

        dataset = json.loads(json.dumps(load_dataset_from_frictionless(path)))
        resource = dataset["resources"][0]

        assert resource["name"] == "my_items"
        assert resource["fileDialect"] == {"format": "csv", "delimiter": ";"}
        assert resource["tableSchema"] == {
            "properties": {"id": {"type": ["integer", "null"]}}
        }

    def test_loads_a_fairspec_dataset_named_datapackage_json_as_it_is(self):
        path = write_package(
            {
                "datapackage.json": {
                    "$schema": "https://fairspec.org/profiles/latest/dataset.json",
                    "resources": [{"name": "items", "data": [{"id": 1}]}],
                }
            }
        )

        dataset = load_dataset_from_frictionless(path)

        assert dataset["resources"] == [{"name": "items", "data": [{"id": 1}]}]

    def test_raises_metadata_errors_for_an_invalid_package(self):
        path = write_package({"datapackage.json": {"resources": [{"bytes": "x"}]}})

        with pytest.raises(FairspecException) as info:
            load_dataset_from_frictionless(path)

        assert info.value.report is not None
        errors = [
            error.model_dump(exclude={"message"}, exclude_none=True)
            for error in info.value.report.errors
        ]
        assert errors == [{"type": "metadata", "jsonPointer": "/resources/0/bytes"}]


def write_package(files: dict[str, object]) -> str:
    folder = get_temp_folder_path()
    os.makedirs(folder, exist_ok=True)
    for name, content in files.items():
        with open(os.path.join(folder, name), "w", encoding="utf-8") as file:
            json.dump(content, file)
    return os.path.join(folder, "datapackage.json")
