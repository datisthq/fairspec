from __future__ import annotations

from unittest.mock import MagicMock, patch

import polars as pl
from fairspec_metadata import Resource, SqliteFileDialect

from ..plugin import SqlitePlugin


class TestSqlitePluginLoadTable:
    def setup_method(self):
        self.plugin = SqlitePlugin()

    @patch("fairspec_table.plugins.sqlite.plugin.load_sqlite_table")
    def test_should_load_table_from_sqlite_file(self, mock_load: MagicMock):
        resource = Resource(data="test.sqlite")
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        assert result is mock_table

    @patch("fairspec_table.plugins.sqlite.plugin.load_sqlite_table")
    def test_should_return_none_for_non_sqlite_files(self, mock_load: MagicMock):
        resource = Resource(data="test.csv")

        result = self.plugin.load_table(resource)

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.sqlite.plugin.load_sqlite_table")
    def test_should_handle_explicit_sqlite_format(self, mock_load: MagicMock):
        resource = Resource(data="test.db", fileDialect=SqliteFileDialect())
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table

        result = self.plugin.load_table(resource)

        assert result is mock_table

    @patch("fairspec_table.plugins.sqlite.plugin.load_sqlite_table")
    def test_should_pass_through_load_options(self, mock_load: MagicMock):
        resource = Resource(data="test.sqlite")
        mock_load.return_value = pl.DataFrame().lazy()

        self.plugin.load_table(resource, denormalized=True)

        mock_load.assert_called_once()

    @patch("fairspec_table.plugins.sqlite.plugin.load_sqlite_table")
    def test_should_handle_paths_with_directories(self, mock_load: MagicMock):
        resource = Resource(data="/path/to/data.sqlite")
        mock_load.return_value = pl.DataFrame().lazy()

        self.plugin.load_table(resource)

        mock_load.assert_called_once()


class TestSqlitePluginSaveTable:
    def setup_method(self):
        self.plugin = SqlitePlugin()

    @patch("fairspec_table.plugins.sqlite.plugin.save_sqlite_table")
    def test_should_save_table_to_sqlite_file(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.sqlite"

        result = self.plugin.save_table(table, path="output.sqlite")

        mock_save.assert_called_once_with(table, path="output.sqlite")
        assert result == "output.sqlite"

    @patch("fairspec_table.plugins.sqlite.plugin.save_sqlite_table")
    def test_should_return_none_for_non_sqlite_files(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output.csv")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.sqlite.plugin.save_sqlite_table")
    def test_should_handle_explicit_sqlite_format(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.db"

        result = self.plugin.save_table(
            table, path="output.db", fileDialect=SqliteFileDialect()
        )

        mock_save.assert_called_once_with(
            table, path="output.db", fileDialect=SqliteFileDialect()
        )
        assert result == "output.db"

    @patch("fairspec_table.plugins.sqlite.plugin.save_sqlite_table")
    def test_should_handle_paths_with_directories(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "/path/to/output.sqlite"

        self.plugin.save_table(table, path="/path/to/output.sqlite")

        mock_save.assert_called_once_with(table, path="/path/to/output.sqlite")

    @patch("fairspec_table.plugins.sqlite.plugin.save_sqlite_table")
    def test_should_return_none_for_files_without_extension(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output")

        mock_save.assert_not_called()
        assert result is None
