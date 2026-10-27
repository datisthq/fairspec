from unittest.mock import MagicMock, patch

from fairspec_metadata import Dataset

from ..plugin import FrictionlessPlugin


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
