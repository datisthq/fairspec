from __future__ import annotations

import polars as pl
from fairspec_dataset import get_temp_file_path

from ..save import save_table


class TestSaveTable:
    def test_should_save_table_to_csv(self):
        path = get_temp_file_path(format="csv")
        table = pl.DataFrame({"id": [1, 2], "name": ["english", "中文"]}).lazy()
        result = save_table(table, path=path)
        assert result is not None

    def test_should_save_table_to_json(self):
        path = get_temp_file_path(format="json")
        table = pl.DataFrame({"id": [1, 2], "name": ["english", "中文"]}).lazy()
        result = save_table(table, path=path)
        assert result is not None

    def test_should_return_none_for_unknown_format(self):
        path = get_temp_file_path(format="unknown")
        table = pl.DataFrame({"id": [1, 2]}).lazy()
        result = save_table(table, path=path)
        assert result is None
