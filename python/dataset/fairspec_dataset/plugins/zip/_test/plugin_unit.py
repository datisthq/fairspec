from unittest.mock import MagicMock, patch

from fairspec_metadata import Dataset
from fairspec_metadata import Resource

from ..plugin import ZipPlugin


class TestLoadDataset:
    def setup_method(self):
        self.plugin = ZipPlugin()

    @patch("fairspec_dataset.plugins.zip.plugin.load_dataset_from_zip")
    def test_loads_from_zip_file(self, mock_load: MagicMock):
        mock_dataset = MagicMock()
        mock_dataset.model_dump.return_value = {
            "resources": [{"name": "test-resource", "data": []}]
        }
        mock_load.return_value = mock_dataset

        result = self.plugin.load_dataset("test.zip")

        mock_load.assert_called_once_with("test.zip")
        assert result == {"resources": [{"name": "test-resource", "data": []}]}

    @patch("fairspec_dataset.plugins.zip.plugin.load_dataset_from_zip")
    def test_returns_none_for_non_zip(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("test.json")

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.zip.plugin.load_dataset_from_zip")
    def test_handles_paths_with_directories(self, mock_load: MagicMock):
        mock_dataset = MagicMock()
        mock_dataset.model_dump.return_value = {"resources": []}
        mock_load.return_value = mock_dataset

        result = self.plugin.load_dataset("/path/to/file.zip")

        mock_load.assert_called_once_with("/path/to/file.zip")
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.zip.plugin.load_dataset_from_zip")
    def test_returns_none_for_no_extension(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("test")

        mock_load.assert_not_called()
        assert result is None


class TestSaveDataset:
    def setup_method(self):
        self.plugin = ZipPlugin()
        self.dataset = Dataset(resources=[Resource(name="test_resource", data=[])])

    @patch("fairspec_dataset.plugins.zip.plugin.save_dataset_to_zip")
    def test_saves_to_zip_file(self, mock_save: MagicMock):
        result = self.plugin.save_dataset(self.dataset, target="output.zip")

        mock_save.assert_called_once_with(
            self.dataset, archive_path="output.zip", with_remote=False
        )
        assert result is not None
        assert result.path is None

    @patch("fairspec_dataset.plugins.zip.plugin.save_dataset_to_zip")
    def test_returns_none_for_non_zip(self, mock_save: MagicMock):
        result = self.plugin.save_dataset(self.dataset, target="output.json")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.zip.plugin.save_dataset_to_zip")
    def test_passes_with_remote_option(self, mock_save: MagicMock):
        result = self.plugin.save_dataset(
            self.dataset, target="output.zip", with_remote=True
        )

        mock_save.assert_called_once_with(
            self.dataset, archive_path="output.zip", with_remote=True
        )
        assert result is not None

    @patch("fairspec_dataset.plugins.zip.plugin.save_dataset_to_zip")
    def test_with_remote_defaults_to_false(self, mock_save: MagicMock):
        self.plugin.save_dataset(self.dataset, target="output.zip")

        mock_save.assert_called_once_with(
            self.dataset, archive_path="output.zip", with_remote=False
        )

    @patch("fairspec_dataset.plugins.zip.plugin.save_dataset_to_zip")
    def test_handles_paths_with_directories(self, mock_save: MagicMock):
        result = self.plugin.save_dataset(self.dataset, target="/path/to/output.zip")

        mock_save.assert_called_once_with(
            self.dataset, archive_path="/path/to/output.zip", with_remote=False
        )
        assert result is not None

    @patch("fairspec_dataset.plugins.zip.plugin.save_dataset_to_zip")
    def test_returns_none_for_no_extension(self, mock_save: MagicMock):
        result = self.plugin.save_dataset(self.dataset, target="output")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.zip.plugin.save_dataset_to_zip")
    def test_result_has_path_none(self, mock_save: MagicMock):
        result = self.plugin.save_dataset(self.dataset, target="output.zip")

        assert result is not None
        assert result.path is None
