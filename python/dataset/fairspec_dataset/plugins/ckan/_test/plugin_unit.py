from unittest.mock import MagicMock, patch

from ..plugin import CkanPlugin


class TestLoadDataset:
    def setup_method(self):
        self.plugin = CkanPlugin()

    @patch("fairspec_dataset.plugins.ckan.plugin.load_dataset_from_ckan")
    def test_loads_from_ckan_url(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("https://demo.ckan.org/dataset/my-dataset")
        mock_load.assert_called_once_with("https://demo.ckan.org/dataset/my-dataset")
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.ckan.plugin.load_dataset_from_ckan")
    def test_returns_none_for_url_without_dataset_path(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://demo.ckan.org/about")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.ckan.plugin.load_dataset_from_ckan")
    def test_returns_none_for_local_paths(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("/tmp/data")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.ckan.plugin.load_dataset_from_ckan")
    def test_returns_none_for_github_urls(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://github.com/owner/repo")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.ckan.plugin.load_dataset_from_ckan")
    def test_handles_additional_path_segments(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset(
            "https://demo.ckan.org/en_GB/dataset/my-dataset"
        )
        mock_load.assert_called_once()
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.ckan.plugin.load_dataset_from_ckan")
    def test_handles_query_parameters(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset(
            "https://demo.ckan.org/dataset/my-dataset?page=1"
        )
        mock_load.assert_called_once()
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.ckan.plugin.load_dataset_from_ckan")
    def test_handles_http_urls(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("http://demo.ckan.org/dataset/my-dataset")
        mock_load.assert_called_once()
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.ckan.plugin.load_dataset_from_ckan")
    def test_returns_none_for_zenodo_urls(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://zenodo.org/records/123456")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.ckan.plugin.load_dataset_from_ckan")
    def test_returns_none_for_dataset_in_query_params_only(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://example.com/search?name=dataset")
        mock_load.assert_not_called()
        assert result is None
