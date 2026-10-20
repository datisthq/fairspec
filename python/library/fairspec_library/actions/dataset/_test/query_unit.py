from __future__ import annotations

import polars as pl
import pytest
from fairspec_metadata import Dataset

from ..query import query_dataset

USERS = [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
POSTS = [{"id": 1, "user_id": 2}, {"id": 2, "user_id": 1}, {"id": 3, "user_id": 2}]


def collect_records(dataset: Dataset, query: str) -> list[dict[str, object]]:
    frame: pl.DataFrame = query_dataset(dataset, query).collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
    return frame.to_dicts()


class TestQueryDataset:
    def test_join_resources_by_name(self):
        dataset = Dataset.model_validate(
            {
                "resources": [
                    {"name": "users", "data": USERS},
                    {"name": "posts", "data": POSTS},
                ]
            }
        )

        records = collect_records(
            dataset,
            "SELECT name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id "
            "GROUP BY name ORDER BY name",
        )

        assert records == [{"name": "Alice", "n": 1}, {"name": "Bob", "n": 2}]

    def test_unnamed_resource_uses_inferred_name(self):
        dataset = Dataset.model_validate({"resources": [{"data": USERS}]})

        records = collect_records(dataset, "SELECT COUNT(*) AS n FROM resource1")

        assert records == [{"n": 2}]

    def test_unreferenced_resource_is_not_loaded(self):
        dataset = Dataset.model_validate(
            {
                "resources": [
                    {"name": "users", "data": USERS},
                    {"name": "broken", "data": "missing.csv"},
                ]
            }
        )

        records = collect_records(dataset, "SELECT COUNT(*) AS n FROM users")

        assert records == [{"n": 2}]

    def test_non_table_resource_is_skipped(self):
        dataset = Dataset.model_validate(
            {
                "resources": [
                    {"name": "users", "data": USERS},
                    {"name": "config", "data": {"key": "value"}},
                ]
            }
        )

        records = collect_records(
            dataset, "SELECT name FROM users WHERE name <> 'config' ORDER BY name"
        )

        assert records == [{"name": "Alice"}, {"name": "Bob"}]

    def test_duplicate_name_raises(self):
        dataset = Dataset.model_validate(
            {
                "resources": [
                    {"name": "users", "data": USERS},
                    {"name": "users", "data": USERS},
                ]
            }
        )

        with pytest.raises(ValueError, match='More than one resource is named "users"'):
            query_dataset(dataset, "SELECT * FROM users")
