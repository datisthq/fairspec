from __future__ import annotations

from unittest.mock import MagicMock, patch

import polars as pl
from fairspec_metadata import ParquetFileDialect, Resource

from ..plugin import ParquetPlugin


class TestParquetPluginLoadTable:
    def setup_method(self):
        self.plugin = ParquetPlugin()

    @patch("fairspec_table.plugins.parquet.plugin.load_parquet_table")
    def test_should_load_table_from_parquet_file(self, mock_load: MagicMock):
        resource = Resource(data="test.parquet")
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        mock_load.assert_called_once_with(resource)
        assert result is mock_table

    @patch("fairspec_table.plugins.parquet.plugin.load_parquet_table")
    def test_should_return_none_for_non_parquet_files(self, mock_load: MagicMock):
        resource = Resource(data="test.csv")

        result = self.plugin.load_table(resource)

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.parquet.plugin.load_parquet_table")
    def test_should_handle_explicit_format(self, mock_load: MagicMock):
        resource = Resource(data="test.txt", fileDialect=ParquetFileDialect())
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        mock_load.assert_called_once_with(resource)
        assert result is mock_table

    @patch("fairspec_table.plugins.parquet.plugin.load_parquet_table")
    def test_should_pass_through_load_options(self, mock_load: MagicMock):
        resource = Resource(data="test.parquet")
        mock_load.return_value = pl.DataFrame().lazy()

        self.plugin.load_table(resource, denormalized=True)

        mock_load.assert_called_once()

    @patch("fairspec_table.plugins.parquet.plugin.load_parquet_table")
    def test_should_handle_paths_with_directories(self, mock_load: MagicMock):
        resource = Resource(data="/path/to/data.parquet")
        mock_load.return_value = pl.DataFrame().lazy()

        self.plugin.load_table(resource)

        mock_load.assert_called_once_with(resource)

    @patch("fairspec_table.plugins.parquet.plugin.load_parquet_table")
    def test_should_return_none_for_arrow_files(self, mock_load: MagicMock):
        resource = Resource(data="test.arrow")

        result = self.plugin.load_table(resource)

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.parquet.plugin.load_parquet_table")
    def test_should_return_none_for_json_files(self, mock_load: MagicMock):
        resource = Resource(data="test.json")

        result = self.plugin.load_table(resource)

        mock_load.assert_not_called()
        assert result is None


class TestParquetPluginSaveTable:
    def setup_method(self):
        self.plugin = ParquetPlugin()

    @patch("fairspec_table.plugins.parquet.plugin.save_parquet_table")
    def test_should_save_table_to_parquet_file(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.parquet"

        result = self.plugin.save_table(table, path="output.parquet")

        mock_save.assert_called_once_with(table, path="output.parquet")
        assert result == "output.parquet"

    @patch("fairspec_table.plugins.parquet.plugin.save_parquet_table")
    def test_should_return_none_for_non_parquet_files(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output.csv")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.parquet.plugin.save_parquet_table")
    def test_should_handle_explicit_format(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.txt"

        result = self.plugin.save_table(
            table, path="output.txt", fileDialect=ParquetFileDialect()
        )

        mock_save.assert_called_once_with(
            table, path="output.txt", fileDialect=ParquetFileDialect()
        )
        assert result == "output.txt"

    @patch("fairspec_table.plugins.parquet.plugin.save_parquet_table")
    def test_should_handle_paths_with_directories(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "/path/to/output.parquet"

        self.plugin.save_table(table, path="/path/to/output.parquet")

        mock_save.assert_called_once_with(table, path="/path/to/output.parquet")

    @patch("fairspec_table.plugins.parquet.plugin.save_parquet_table")
    def test_should_return_none_for_files_without_extension(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.parquet.plugin.save_parquet_table")
    def test_should_return_none_for_arrow_files(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output.arrow")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.parquet.plugin.save_parquet_table")
    def test_should_return_none_for_json_files(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output.json")

        mock_save.assert_not_called()
        assert result is None
