from __future__ import annotations

import polars as pl

from ..query import query_table, query_tables, set_nulls_last


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


class TestSetNullsLast:
    def test_should_sort_nulls_last_in_descending_order(self):
        table = pl.DataFrame({"x": [1, None, 2]}).lazy()

        result = query_table(table, "SELECT x FROM self ORDER BY x DESC")

        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.get_column("x").to_list() == [2, 1, None]

    def test_should_keep_an_explicit_null_order(self):
        query = "SELECT x FROM self ORDER BY x DESC NULLS FIRST, y"

        assert set_nulls_last(query) == (
            "SELECT x FROM self ORDER BY x DESC NULLS FIRST, y NULLS LAST"
        )

    def test_should_end_items_at_parentheses_and_limits(self):
        query = "SELECT * FROM (SELECT x FROM self ORDER BY f(x) LIMIT 2) ORDER BY 1"

        assert set_nulls_last(query) == (
            "SELECT * FROM (SELECT x FROM self ORDER BY f(x) NULLS LAST LIMIT 2)"
            " ORDER BY 1 NULLS LAST"
        )

    def test_should_skip_quoted_text_and_comments(self):
        query = "SELECT 'order by' AS \"ORDER BY\" FROM self -- order by x\nORDER BY a"

        assert set_nulls_last(query) == (
            "SELECT 'order by' AS \"ORDER BY\" FROM self -- order by x\n"
            "ORDER BY a NULLS LAST"
        )
