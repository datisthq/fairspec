from __future__ import annotations

from datetime import datetime

import polars as pl
import pytest
from fairspec_metadata import IntegerColumnProperty
from fairspec_metadata import StringColumnProperty
from fairspec_metadata import Resource
from fairspec_metadata import TableSchema

from ..load import load_inline_table


class TestLoadInlineTable:
    def test_should_raise_on_no_data(self):
        resource = Resource(name="test")

        with pytest.raises(Exception, match="Resource data is not defined or tabular"):
            load_inline_table(resource)

    def test_should_raise_on_bad_data(self):
        resource = Resource(name="test", data="bad")

        with pytest.raises(Exception, match="Resource data is not defined or tabular"):
            load_inline_table(resource)

    def test_should_read_table_data(self):
        resource = Resource(
            name="test",
            data=[
                {"id": 1, "name": "english"},
                {"id": 2, "name": "中文"},
            ],
        )

        table = load_inline_table(resource)
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_handle_longer_rows(self):
        resource = Resource(
            data=[
                {"id": 1, "name": "english"},
                {"id": 2, "name": "中文", "extra": "bad"},
            ],
            tableSchema=TableSchema(
                properties={
                    "id": IntegerColumnProperty(),
                    "name": StringColumnProperty(),
                },
            ),
        )

        table = load_inline_table(resource)
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_handle_shorter_rows(self):
        resource = Resource(
            name="test",
            data=[{"id": 1, "name": "english"}, {"id": 2}],
            tableSchema=TableSchema(
                properties={
                    "id": IntegerColumnProperty(),
                    "name": StringColumnProperty(),
                },
            ),
        )

        table = load_inline_table(resource)
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": None},
        ]

    def test_should_handle_various_data_types(self):
        resource = Resource(
            data=[
                {
                    "string": "string",
                    "number": 1,
                    "boolean": True,
                    "date": datetime(2025, 1, 1),
                    "time": datetime(2025, 1, 1),
                    "datetime": datetime(2025, 1, 1),
                },
            ],
        )

        table = load_inline_table(resource)
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278

        assert frame.to_dicts() == [
            {
                "string": "string",
                "number": 1,
                "boolean": True,
                "date": datetime(2025, 1, 1),
                "time": datetime(2025, 1, 1),
                "datetime": datetime(2025, 1, 1),
            },
        ]

    def test_should_handle_objects_with_shorter_rows(self):
        resource = Resource(
            data=[
                {"id": 1, "name": "english"},
                {"id": 2, "name": "中文"},
                {"id": 3},
            ],
        )

        table = load_inline_table(resource)
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
            {"id": 3, "name": None},
        ]

    def test_should_handle_objects_with_longer_rows(self):
        resource = Resource(
            data=[
                {"id": 1, "name": "english"},
                {"id": 2, "name": "中文"},
                {"id": 3, "name": "german", "extra": "extra"},
            ],
        )

        table = load_inline_table(resource)
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278

        assert frame.to_dicts() == [
            {"id": 1, "name": "english", "extra": None},
            {"id": 2, "name": "中文", "extra": None},
            {"id": 3, "name": "german", "extra": "extra"},
        ]
