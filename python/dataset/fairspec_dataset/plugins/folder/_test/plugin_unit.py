from unittest.mock import MagicMock, patch

from fairspec_metadata import Dataset
from fairspec_metadata import Resource

from ..plugin import FolderPlugin


class TestLoadDataset:
    def setup_method(self):
        self.plugin = FolderPlugin()

    @patch("fairspec_dataset.plugins.folder.plugin.os.path.isdir", return_value=True)
    @patch("fairspec_dataset.plugins.folder.plugin.load_dataset_from_folder")
    def test_loads_from_local_directory(
        self, mock_load: MagicMock, _mock_isdir: MagicMock
    ):
        mock_dataset = MagicMock()
        mock_dataset.model_dump.return_value = {
            "resources": [{"name": "test", "data": []}]
        }
        mock_load.return_value = mock_dataset

        result = self.plugin.load_dataset(".")

        mock_load.assert_called_once_with(".")
        assert result == {"resources": [{"name": "test", "data": []}]}

    @patch("fairspec_dataset.plugins.folder.plugin.load_dataset_from_folder")
    def test_returns_none_for_http(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("http://example.com/data")

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.folder.plugin.load_dataset_from_folder")
    def test_returns_none_for_https(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://example.com/data")

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.folder.plugin.load_dataset_from_folder")
    def test_returns_none_for_ftp(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("ftp://example.com/data")

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.folder.plugin.load_dataset_from_folder")
    def test_returns_none_for_github(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://github.com/owner/repo/data")

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.folder.plugin.load_dataset_from_folder")
    def test_returns_none_for_zenodo(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://zenodo.org/record/123")

        mock_load.assert_not_called()
        assert result is None


class TestSaveDataset:
    def setup_method(self):
        self.plugin = FolderPlugin()
        self.dataset = Dataset(resources=[Resource(name="test", data=[])])

    @patch("fairspec_dataset.plugins.folder.plugin.os.path.isdir", return_value=True)
    @patch("fairspec_dataset.plugins.folder.plugin.save_dataset_to_folder")
    def test_saves_to_local_directory(self, mock_save: MagicMock, _mock_isdir: MagicMock):
        result = self.plugin.save_dataset(self.dataset, target="/tmp/test")

        mock_save.assert_called_once_with(
            self.dataset, folder_path="/tmp/test", with_remote=False
        )
        assert result is not None
        assert result.path == "/tmp/test"

    @patch("fairspec_dataset.plugins.folder.plugin.os.path.isdir", return_value=True)
    @patch("fairspec_dataset.plugins.folder.plugin.save_dataset_to_folder")
    def test_saves_with_remote_option(self, mock_save: MagicMock, _mock_isdir: MagicMock):
        result = self.plugin.save_dataset(
            self.dataset, target="/tmp/test", with_remote=True
        )

        mock_save.assert_called_once_with(
            self.dataset, folder_path="/tmp/test", with_remote=True
        )
        assert result is not None
        assert result.path == "/tmp/test"

    @patch("fairspec_dataset.plugins.folder.plugin.save_dataset_to_folder")
    def test_returns_none_for_http(self, mock_save: MagicMock):
        result = self.plugin.save_dataset(self.dataset, target="http://example.com/data")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.folder.plugin.save_dataset_to_folder")
    def test_returns_none_for_https(self, mock_save: MagicMock):
        result = self.plugin.save_dataset(self.dataset, target="https://example.com/data")

        mock_save.assert_not_called()
        assert result is None
