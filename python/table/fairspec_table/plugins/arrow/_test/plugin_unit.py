from __future__ import annotations

from unittest.mock import MagicMock, patch

import polars as pl
from fairspec_metadata import ArrowFileDialect, Resource

from ..plugin import ArrowPlugin


class TestArrowPluginLoadTable:
    def setup_method(self):
        self.plugin = ArrowPlugin()

    @patch("fairspec_table.plugins.arrow.plugin.load_arrow_table")
    def test_should_load_table_from_arrow_file(self, mock_load: MagicMock):
        resource = Resource(data="test.arrow")
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        mock_load.assert_called_once_with(resource)
        assert result is mock_table

    @patch("fairspec_table.plugins.arrow.plugin.load_arrow_table")
    def test_should_load_table_from_feather_file(self, mock_load: MagicMock):
        resource = Resource(data="test.feather")
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        mock_load.assert_called_once_with(resource)
        assert result is mock_table

    @patch("fairspec_table.plugins.arrow.plugin.load_arrow_table")
    def test_should_return_none_for_non_arrow_files(self, mock_load: MagicMock):
        resource = Resource(data="test.csv")

        result = self.plugin.load_table(resource)

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.arrow.plugin.load_arrow_table")
    def test_should_handle_explicit_arrow_format(self, mock_load: MagicMock):
        resource = Resource(data="test.txt", fileDialect=ArrowFileDialect())
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        mock_load.assert_called_once_with(resource)
        assert result is mock_table

    @patch("fairspec_table.plugins.arrow.plugin.load_arrow_table")
    def test_should_pass_through_load_options(self, mock_load: MagicMock):
        resource = Resource(data="test.arrow")
        mock_load.return_value = pl.DataFrame().lazy()

        self.plugin.load_table(resource, denormalized=True)

        mock_load.assert_called_once()

    @patch("fairspec_table.plugins.arrow.plugin.load_arrow_table")
    def test_should_handle_paths_with_directories(self, mock_load: MagicMock):
        resource = Resource(data="/path/to/data.arrow")
        mock_load.return_value = pl.DataFrame().lazy()

        self.plugin.load_table(resource)

        mock_load.assert_called_once_with(resource)

    @patch("fairspec_table.plugins.arrow.plugin.load_arrow_table")
    def test_should_return_none_for_parquet_files(self, mock_load: MagicMock):
        resource = Resource(data="test.parquet")

        result = self.plugin.load_table(resource)

        mock_load.assert_not_called()
        assert result is None


class TestArrowPluginSaveTable:
    def setup_method(self):
        self.plugin = ArrowPlugin()

    @patch("fairspec_table.plugins.arrow.plugin.save_arrow_table")
    def test_should_save_table_to_arrow_file(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.arrow"

        result = self.plugin.save_table(table, path="output.arrow")

        mock_save.assert_called_once_with(table, path="output.arrow")
        assert result == "output.arrow"

    @patch("fairspec_table.plugins.arrow.plugin.save_arrow_table")
    def test_should_save_table_to_feather_file(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.feather"

        result = self.plugin.save_table(table, path="output.feather")

        mock_save.assert_called_once_with(table, path="output.feather")
        assert result == "output.feather"

    @patch("fairspec_table.plugins.arrow.plugin.save_arrow_table")
    def test_should_return_none_for_non_arrow_files(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output.csv")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.arrow.plugin.save_arrow_table")
    def test_should_handle_explicit_arrow_format(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.txt"

        result = self.plugin.save_table(
            table, path="output.txt", fileDialect=ArrowFileDialect()
        )

        mock_save.assert_called_once_with(
            table, path="output.txt", fileDialect=ArrowFileDialect()
        )
        assert result == "output.txt"

    @patch("fairspec_table.plugins.arrow.plugin.save_arrow_table")
    def test_should_handle_paths_with_directories(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "/path/to/output.arrow"

        self.plugin.save_table(table, path="/path/to/output.arrow")

        mock_save.assert_called_once_with(table, path="/path/to/output.arrow")

    @patch("fairspec_table.plugins.arrow.plugin.save_arrow_table")
    def test_should_return_none_for_files_without_extension(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.arrow.plugin.save_arrow_table")
    def test_should_return_none_for_parquet_files(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output.parquet")

        mock_save.assert_not_called()
        assert result is None
