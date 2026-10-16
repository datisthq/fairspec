from __future__ import annotations

import json
import os

from fairspec_dataset import get_temp_folder_path

from ..load import load_dataset


class TestLoadDataset:
    def test_should_load_dataset_from_descriptor_path(self):
        folder = get_temp_folder_path()
        descriptor = {"resources": [{"data": "data.csv", "name": "data"}]}
        path = os.path.join(folder, "datapackage.json")
        with open(path, "w") as f:
            json.dump(descriptor, f)
        result = load_dataset(path)
        assert result is not None
        assert "resources" in result

    def test_should_return_none_for_unsupported_source(self):
        result = load_dataset("nonexistent-source")
        assert result is None
