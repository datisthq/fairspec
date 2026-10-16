from __future__ import annotations

import polars as pl
from fairspec_dataset import write_temp_file
from fairspec_metadata import Resource

from ..load import load_table


class TestLoadTable:
    def test_should_load_csv_table(self):
        path = write_temp_file("id,name\n1,english\n2,中文", format="csv")
        resource = Resource(data=path)
        table = load_table(resource)
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_load_inline_table(self):
        resource = Resource(
            data=[{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}]
        )
        table = load_table(resource)
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_return_none_for_empty_resource(self):
        resource = Resource()
        table = load_table(resource)
        assert table is None

    def test_should_load_json_table(self):
        path = write_temp_file('[{"id": 1, "name": "english"}]', format="json")
        resource = Resource(data=path)
        table = load_table(resource)
        assert table is not None
