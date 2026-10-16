from __future__ import annotations

import os

from fairspec_dataset import get_temp_file_path
from fairspec_metadata import Dataset, Resource

from ..save import save_dataset


class TestSaveDataset:
    def test_should_save_dataset_to_zip(self):
        path = get_temp_file_path(format="zip")
        dataset = Dataset(resources=[Resource(data=[{"id": 1}], name="data")])
        result = save_dataset(dataset, target=path)
        assert result is not None

    def test_should_save_dataset_to_folder(self):
        path = get_temp_file_path()
        dataset = Dataset(resources=[Resource(data=[{"id": 1}], name="data")])
        result = save_dataset(dataset, target=path)
        assert result is not None
        assert os.path.isfile(os.path.join(path, "dataset.json"))

    def test_should_return_none_for_unsupported_target(self):
        dataset = Dataset(resources=[Resource(data=[{"id": 1}], name="data")])
        result = save_dataset(dataset, target="/tmp/unknown.xyz")
        assert result is None
