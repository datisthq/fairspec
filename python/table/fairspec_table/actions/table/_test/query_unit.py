from __future__ import annotations

import polars as pl

from ..query import query_table, query_tables


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


class TestQueryTables:
    def test_join_tables_by_name(self):
        users = pl.DataFrame({"id": [1, 2], "name": ["Alice", "Bob"]}).lazy()
        posts = pl.DataFrame({"user_id": [2, 1, 2], "title": ["a", "b", "c"]}).lazy()

        result = query_tables(
            {"users": users, "posts": posts},
            "SELECT name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id "
            "GROUP BY name ORDER BY name",
        )

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == [{"name": "Alice", "n": 1}, {"name": "Bob", "n": 2}]
