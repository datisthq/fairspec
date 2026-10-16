from __future__ import annotations

from unittest.mock import MagicMock, patch

import polars as pl
from fairspec_metadata import JsonFileDialect, JsonlFileDialect, Resource

from ..plugin import JsonPlugin


class TestJsonPluginLoadTable:
    def setup_method(self):
        self.plugin = JsonPlugin()

    @patch("fairspec_table.plugins.json.plugin.infer_json_file_dialect")
    @patch("fairspec_table.plugins.json.plugin.load_json_table")
    def test_should_load_table_from_json_file(
        self, mock_load: MagicMock, mock_infer: MagicMock
    ):
        resource = Resource(data="test.json")
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table
        mock_infer.return_value = JsonFileDialect()

        result = self.plugin.load_table(resource)

        assert result is mock_table

    @patch("fairspec_table.plugins.json.plugin.infer_json_file_dialect")
    @patch("fairspec_table.plugins.json.plugin.load_json_table")
    def test_should_load_table_from_jsonl_file(
        self, mock_load: MagicMock, mock_infer: MagicMock
    ):
        resource = Resource(data="test.jsonl")
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table
        mock_infer.return_value = JsonlFileDialect()

        result = self.plugin.load_table(resource)

        assert result is mock_table

    @patch("fairspec_table.plugins.json.plugin.infer_json_file_dialect")
    @patch("fairspec_table.plugins.json.plugin.load_json_table")
    def test_should_load_table_from_ndjson_file(
        self, mock_load: MagicMock, mock_infer: MagicMock
    ):
        resource = Resource(data="test.ndjson")
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table
        mock_infer.return_value = JsonlFileDialect()

        result = self.plugin.load_table(resource)

        assert result is mock_table

    @patch("fairspec_table.plugins.json.plugin.load_json_table")
    def test_should_return_none_for_non_json_files(self, mock_load: MagicMock):
        resource = Resource(data="test.csv")

        result = self.plugin.load_table(resource)

        mock_load.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.json.plugin.infer_json_file_dialect")
    @patch("fairspec_table.plugins.json.plugin.load_json_table")
    def test_should_handle_explicit_json_format(
        self, mock_load: MagicMock, mock_infer: MagicMock
    ):
        resource = Resource(data="test.txt", fileDialect=JsonFileDialect())
        mock_table = pl.DataFrame().lazy()
        mock_load.return_value = mock_table
        mock_infer.return_value = JsonFileDialect()

        result = self.plugin.load_table(resource)

        assert result is mock_table

    @patch("fairspec_table.plugins.json.plugin.infer_json_file_dialect")
    @patch("fairspec_table.plugins.json.plugin.load_json_table")
    def test_should_pass_through_load_options(
        self, mock_load: MagicMock, mock_infer: MagicMock
    ):
        resource = Resource(data="test.json")
        mock_load.return_value = pl.DataFrame().lazy()
        mock_infer.return_value = JsonFileDialect()

        self.plugin.load_table(resource, denormalized=True)

        mock_load.assert_called_once()

    @patch("fairspec_table.plugins.json.plugin.infer_json_file_dialect")
    @patch("fairspec_table.plugins.json.plugin.load_json_table")
    def test_should_handle_paths_with_directories(
        self, mock_load: MagicMock, mock_infer: MagicMock
    ):
        resource = Resource(data="/path/to/data.json")
        mock_load.return_value = pl.DataFrame().lazy()
        mock_infer.return_value = JsonFileDialect()

        self.plugin.load_table(resource)

        mock_load.assert_called_once()

    @patch("fairspec_table.plugins.json.plugin.load_json_table")
    def test_should_return_none_for_parquet_files(self, mock_load: MagicMock):
        resource = Resource(data="test.parquet")

        result = self.plugin.load_table(resource)

        mock_load.assert_not_called()
        assert result is None


class TestJsonPluginInferFileDialect:
    def setup_method(self):
        self.plugin = JsonPlugin()

    @patch("fairspec_table.plugins.json.plugin.infer_json_file_dialect")
    def test_should_infer_dialect_for_json_file(self, mock_infer: MagicMock):
        resource = Resource(data="test.json")
        mock_infer.return_value = JsonFileDialect()

        result = self.plugin.infer_file_dialect(resource)

        assert result == JsonFileDialect()

    @patch("fairspec_table.plugins.json.plugin.infer_json_file_dialect")
    def test_should_infer_dialect_for_jsonl_file(self, mock_infer: MagicMock):
        resource = Resource(data="test.jsonl")
        mock_infer.return_value = JsonlFileDialect()

        result = self.plugin.infer_file_dialect(resource)

        assert result == JsonlFileDialect()

    @patch("fairspec_table.plugins.json.plugin.infer_json_file_dialect")
    def test_should_return_none_for_non_json_files(self, mock_infer: MagicMock):
        resource = Resource(data="test.csv")

        result = self.plugin.infer_file_dialect(resource)

        mock_infer.assert_not_called()
        assert result is None


class TestJsonPluginSaveTable:
    def setup_method(self):
        self.plugin = JsonPlugin()

    @patch("fairspec_table.plugins.json.plugin.save_json_table")
    def test_should_save_table_to_json_file(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.json"

        result = self.plugin.save_table(table, path="output.json")

        mock_save.assert_called_once_with(table, path="output.json")
        assert result == "output.json"

    @patch("fairspec_table.plugins.json.plugin.save_json_table")
    def test_should_save_table_to_jsonl_file(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.jsonl"

        result = self.plugin.save_table(table, path="output.jsonl")

        mock_save.assert_called_once_with(table, path="output.jsonl")
        assert result == "output.jsonl"

    @patch("fairspec_table.plugins.json.plugin.save_json_table")
    def test_should_save_table_to_ndjson_file(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.ndjson"

        result = self.plugin.save_table(table, path="output.ndjson")

        mock_save.assert_called_once_with(table, path="output.ndjson")
        assert result == "output.ndjson"

    @patch("fairspec_table.plugins.json.plugin.save_json_table")
    def test_should_return_none_for_non_json_files(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output.csv")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.json.plugin.save_json_table")
    def test_should_handle_explicit_json_format(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "output.txt"

        result = self.plugin.save_table(
            table, path="output.txt", fileDialect=JsonFileDialect()
        )

        mock_save.assert_called_once_with(
            table, path="output.txt", fileDialect=JsonFileDialect()
        )
        assert result == "output.txt"

    @patch("fairspec_table.plugins.json.plugin.save_json_table")
    def test_should_handle_paths_with_directories(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()
        mock_save.return_value = "/path/to/output.json"

        self.plugin.save_table(table, path="/path/to/output.json")

        mock_save.assert_called_once_with(table, path="/path/to/output.json")

    @patch("fairspec_table.plugins.json.plugin.save_json_table")
    def test_should_return_none_for_files_without_extension(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output")

        mock_save.assert_not_called()
        assert result is None

    @patch("fairspec_table.plugins.json.plugin.save_json_table")
    def test_should_return_none_for_parquet_files(self, mock_save: MagicMock):
        table = pl.DataFrame().lazy()

        result = self.plugin.save_table(table, path="output.parquet")

        mock_save.assert_not_called()
        assert result is None
