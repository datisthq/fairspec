from unittest.mock import MagicMock, patch

from fairspec_metadata import Dataset

from ..plugin import FrictionlessPlugin


class TestLoadDataset:
    def setup_method(self):
        self.plugin = FrictionlessPlugin()

    @patch("fairspec_dataset.plugins.frictionless.plugin.load_dataset_from_frictionless")
    def test_loads_a_local_datapackage_json(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("./data/datapackage.json")
        mock_load.assert_called_once_with("./data/datapackage.json")
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.frictionless.plugin.load_dataset_from_frictionless")
    def test_loads_a_remote_datapackage_json(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        self.plugin.load_dataset("https://example.com/datapackage.json")
        mock_load.assert_called_once_with("https://example.com/datapackage.json")

    @patch("fairspec_dataset.plugins.frictionless.plugin.load_dataset_from_frictionless")
    def test_does_not_load_a_json_file_with_another_name(self, mock_load: MagicMock):
        assert self.plugin.load_dataset("./dataset.json") is None
        mock_load.assert_not_called()

    @patch("fairspec_dataset.plugins.frictionless.plugin.load_dataset_from_frictionless")
    def test_does_not_load_a_folder_or_a_csv_file(self, mock_load: MagicMock):
        assert self.plugin.load_dataset("./data") is None
        assert self.plugin.load_dataset("./table.csv") is None
        mock_load.assert_not_called()


class TestSaveDataset:
    def setup_method(self):
        self.plugin = FrictionlessPlugin()
        self.dataset = Dataset(resources=[])

    @patch("fairspec_dataset.plugins.frictionless.plugin.save_dataset_to_frictionless")
    def test_saves_to_a_local_datapackage_json(self, mock_save: MagicMock):
        self.plugin.save_dataset(self.dataset, target="./datapackage.json")
        mock_save.assert_called_once_with(self.dataset, target="./datapackage.json")

    def test_does_not_save_to_a_remote_datapackage_json(self):
        result = self.plugin.save_dataset(
            self.dataset, target="https://example.com/datapackage.json"
        )
        assert result is None

    def test_does_not_save_to_a_json_file_with_another_name(self):
        assert self.plugin.save_dataset(self.dataset, target="./dataset.json") is None
