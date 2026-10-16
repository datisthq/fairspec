from unittest.mock import MagicMock, patch

from ..plugin import GithubPlugin


class TestLoadDataset:
    def setup_method(self):
        self.plugin = GithubPlugin()

    @patch("fairspec_dataset.plugins.github.plugin.load_dataset_from_github")
    def test_loads_from_github_url(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("https://github.com/owner/repo")
        mock_load.assert_called_once_with("https://github.com/owner/repo")
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.github.plugin.load_dataset_from_github")
    def test_returns_none_for_non_github_urls(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://example.com/data")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.github.plugin.load_dataset_from_github")
    def test_returns_none_for_local_paths(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("/tmp/data")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.github.plugin.load_dataset_from_github")
    def test_returns_none_for_zenodo_urls(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://zenodo.org/records/123456")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.github.plugin.load_dataset_from_github")
    def test_handles_github_urls_with_paths(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("https://github.com/owner/repo/tree/main")
        mock_load.assert_called_once()
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.github.plugin.load_dataset_from_github")
    def test_handles_github_urls_with_query_params(self, mock_load: MagicMock):
        mock_load.return_value = {"resources": []}
        result = self.plugin.load_dataset("https://github.com/owner/repo?tab=code")
        mock_load.assert_called_once()
        assert result == {"resources": []}

    @patch("fairspec_dataset.plugins.github.plugin.load_dataset_from_github")
    def test_returns_none_for_http_non_github_urls(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("http://example.com/data")
        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_dataset.plugins.github.plugin.load_dataset_from_github")
    def test_returns_none_for_gitlab_urls(self, mock_load: MagicMock):
        result = self.plugin.load_dataset("https://gitlab.com/owner/repo")
        mock_load.assert_not_called()
        assert result is None
