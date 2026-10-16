from __future__ import annotations

import polars as pl

from ..query import query_table


class TestQueryTable:
    def test_select_all(self):
        table = pl.DataFrame({"name": ["Alice", "Bob"], "age": [30, 25]}).lazy()

        result = query_table(table, "SELECT * FROM self")

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25},
        ]

    def test_select_with_filter(self):
        table = pl.DataFrame({"name": ["Alice", "Bob"], "age": [30, 25]}).lazy()

        result = query_table(table, "SELECT name FROM self WHERE age > 26")

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": "Alice"}]

    def test_returns_lazy_frame(self):
        table = pl.DataFrame({"x": [1]}).lazy()

        result = query_table(table, "SELECT * FROM self")

        assert isinstance(result, pl.LazyFrame)
