from unittest.mock import MagicMock, patch

from ..plugin import ZenodoPlugin


class TestLoadDataset:
    def setup_method(self):
        self.plugin = ZenodoPlugin()

    @patch("fairspec_dataset.plugins.zenodo.plugin.load_dataset_from_zenodo")
    def test_loads_from_zenodo_url(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("https://zenodo.org/records/123456")
        mock_load.assert_called_once_with("https://zenodo.org/records/123456")
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.zenodo.plugin.load_dataset_from_zenodo")
    def test_returns_none_for_non_zenodo_urls(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://example.com/data")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.zenodo.plugin.load_dataset_from_zenodo")
    def test_returns_none_for_local_paths(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("/tmp/data")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.zenodo.plugin.load_dataset_from_zenodo")
    def test_returns_none_for_github_urls(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://github.com/owner/repo")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.zenodo.plugin.load_dataset_from_zenodo")
    def test_handles_sandbox_zenodo(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("https://sandbox.zenodo.org/records/123456")
        mock_load.assert_called_once()
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.zenodo.plugin.load_dataset_from_zenodo")
    def test_handles_zenodo_urls_with_paths(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("https://zenodo.org/records/123456/files")
        mock_load.assert_called_once()
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.zenodo.plugin.load_dataset_from_zenodo")
    def test_handles_zenodo_urls_with_query_params(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("https://zenodo.org/records/123456?preview=1")
        mock_load.assert_called_once()
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.zenodo.plugin.load_dataset_from_zenodo")
    def test_returns_none_for_http_non_zenodo_urls(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("http://example.com/data")
        mock_load.assert_not_called()
        assert result is None
