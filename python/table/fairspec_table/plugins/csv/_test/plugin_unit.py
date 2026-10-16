from __future__ import annotations

from unittest.mock import MagicMock, patch

import polars as pl
from fairspec_metadata import CsvFileDialect, Resource, TsvFileDialect

from ..plugin import CsvPlugin


class TestCsvPluginLoadTable:
    def setup_method(self):
        self.plugin = CsvPlugin()

    @patch("fairspec_table.plugins.csv.plugin.load_csv_table")
    def test_should_load_table_from_csv_file(self, mock_load: MagicMock):
        resource = Resource(data="test.csv")
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        assert result is mock_table

    @patch("fairspec_table.plugins.csv.plugin.load_csv_table")
    def test_should_load_table_from_tsv_file(self, mock_load: MagicMock):
        resource = Resource(data="test.tsv")
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        assert result is mock_table

    @patch("fairspec_table.plugins.csv.plugin.load_csv_table")
    def test_should_return_none_for_non_csv_files(self, mock_load: MagicMock):
        resource = Resource(data="test.json")

        result = self.plugin.load_table(resource)

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.csv.plugin.load_csv_table")
    def test_should_handle_explicit_csv_format(self, mock_load: MagicMock):
        resource = Resource(data="test.txt", fileDialect=CsvFileDialect())
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        assert result is mock_table

    @patch("fairspec_table.plugins.csv.plugin.load_csv_table")
    def test_should_pass_through_load_options(self, mock_load: MagicMock):
        resource = Resource(data="test.csv")
        mock_load.return_value = pl.DataFrame().lazy()

        self.plugin.load_table(resource, denormalized=True)

        mock_load.assert_called_once()

    @patch("fairspec_table.plugins.csv.plugin.load_csv_table")
    def test_should_handle_paths_with_directories(self, mock_load: MagicMock):
        resource = Resource(data="/path/to/data.csv")
        mock_load.return_value = pl.DataFrame().lazy()

        self.plugin.load_table(resource)

        mock_load.assert_called_once()

    @patch("fairspec_table.plugins.csv.plugin.load_csv_table")
    def test_should_handle_explicit_tsv_format(self, mock_load: MagicMock):
        resource = Resource(data="test.txt", fileDialect=TsvFileDialect())
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        assert result is mock_table


class TestCsvPluginSaveTable:
    def setup_method(self):
        self.plugin = CsvPlugin()

    @patch("fairspec_table.plugins.csv.plugin.save_csv_table")
    def test_should_save_table_to_csv_file(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.csv"

        result = self.plugin.save_table(table, path="output.csv")

        mock_save.assert_called_once_with(table, path="output.csv")
        assert result == "output.csv"

    @patch("fairspec_table.plugins.csv.plugin.save_csv_table")
    def test_should_save_table_to_tsv_file(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.tsv"

        result = self.plugin.save_table(table, path="output.tsv")

        mock_save.assert_called_once_with(table, path="output.tsv")
        assert result == "output.tsv"

    @patch("fairspec_table.plugins.csv.plugin.save_csv_table")
    def test_should_return_none_for_non_csv_files(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output.json")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.csv.plugin.save_csv_table")
    def test_should_handle_explicit_csv_format(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.txt"

        result = self.plugin.save_table(
            table, path="output.txt", fileDialect=CsvFileDialect()
        )

        mock_save.assert_called_once_with(
            table, path="output.txt", fileDialect=CsvFileDialect()
        )
        assert result == "output.txt"

    @patch("fairspec_table.plugins.csv.plugin.save_csv_table")
    def test_should_handle_paths_with_directories(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "/path/to/output.csv"

        self.plugin.save_table(table, path="/path/to/output.csv")

        mock_save.assert_called_once_with(table, path="/path/to/output.csv")

    @patch("fairspec_table.plugins.csv.plugin.save_csv_table")
    def test_should_return_none_for_files_without_extension(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.csv.plugin.save_csv_table")
    def test_should_handle_explicit_tsv_format(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.txt"

        result = self.plugin.save_table(
            table, path="output.txt", fileDialect=TsvFileDialect()
        )

        mock_save.assert_called_once_with(
            table, path="output.txt", fileDialect=TsvFileDialect()
        )
        assert result == "output.txt"
