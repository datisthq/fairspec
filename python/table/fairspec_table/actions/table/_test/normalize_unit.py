from __future__ import annotations

import polars as pl
from fairspec_metadata import BooleanColumnProperty
from fairspec_metadata import IntegerColumnProperty
from fairspec_metadata import StringColumnProperty
from fairspec_metadata import TableSchema

from ..normalize import normalize_table


class TestNormalizeTable:
    def test_should_work_with_schema(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "name": ["english", "中文"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        records = [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records

    def test_should_work_with_less_fields_in_data(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "name": ["english", "中文"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
                "other": BooleanColumnProperty(),
            }
        )

        records = [
            {"id": 1, "name": "english", "other": None},
            {"id": 2, "name": "中文", "other": None},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records

    def test_should_work_with_more_fields_in_data(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "name": ["english", "中文"],
                "other": [True, False],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        records = [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records

    def test_should_not_work_based_on_fields_order(self):
        table = pl.DataFrame(
            {
                "field1": [1, 2],
                "field2": ["english", "中文"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        records = [
            {"id": None, "name": None},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records

    def test_should_work_based_on_field_names_equal(self):
        table = pl.DataFrame(
            {
                "name": ["english", "中文"],
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        records = [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records

    def test_should_work_based_on_field_names_subset(self):
        table = pl.DataFrame(
            {
                "name": ["english", "中文"],
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        records = [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records

    def test_should_work_based_on_field_names_superset(self):
        table = pl.DataFrame(
            {
                "name": ["english", "中文"],
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        records = [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records

    def test_should_work_based_on_field_names_partial(self):
        table = pl.DataFrame(
            {
                "name": ["english", "中文"],
                "id": [1, 2],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        records = [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records

    def test_should_parse_string_columns(self):
        table = pl.DataFrame(
            {
                "id": ["1", "2"],
                "name": ["english", "中文"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": StringColumnProperty(),
            }
        )

        records = [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records

    def test_should_read_type_errors_as_nulls(self):
        table = pl.DataFrame(
            {
                "id": [1, 2],
                "name": ["english", "中文"],
            }
        ).lazy()

        table_schema = TableSchema(
            properties={
                "id": IntegerColumnProperty(),
                "name": IntegerColumnProperty(),
            }
        )

        records = [
            {"id": 1, "name": None},
            {"id": 2, "name": None},
        ]

        result = normalize_table(table, table_schema)
        frame: pl.DataFrame = result.collect()  # ty: ignore[invalid-assignment] https://github.com/astral-sh/ty/issues/2278
        assert frame.to_dicts() == records
