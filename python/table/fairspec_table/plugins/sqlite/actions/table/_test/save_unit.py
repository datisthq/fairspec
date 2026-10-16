from __future__ import annotations

import polars as pl
from fairspec_dataset import get_temp_file_path
from fairspec_metadata import Resource, SqliteFileDialect

from ..load import load_sqlite_table
from ..save import save_sqlite_table

DIALECT = SqliteFileDialect(tableName="fairspec")


class TestSaveSqliteTable:
    def test_should_save_and_load_table(self):
        path = get_temp_file_path()

        source = pl.DataFrame(
            [{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}]
        ).lazy()
        save_sqlite_table(source, path=path, fileDialect=DIALECT, overwrite=True)

        target = load_sqlite_table(Resource(data=path, fileDialect=DIALECT))
        frame: pl.DataFrame = target.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_save_and_load_table_with_protocol(self):
        path = f"sqlite://{get_temp_file_path()}"

        source = pl.DataFrame(
            [{"id": 1, "name": "english"}, {"id": 2, "name": "中文"}]
        ).lazy()
        save_sqlite_table(source, path=path, fileDialect=DIALECT, overwrite=True)

        target = load_sqlite_table(Resource(data=path, fileDialect=DIALECT))
        frame: pl.DataFrame = target.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_save_and_load_various_data_types(self):
        path = f"sqlite://{get_temp_file_path()}"

        source = pl.DataFrame(
            [
                pl.Series("boolean", [True], dtype=pl.Boolean),
                pl.Series("date", ["2025-01-01"], dtype=pl.String),
                pl.Series("datetime", ["2025-01-01T00:00:00"], dtype=pl.String),
                pl.Series("integer", [1], dtype=pl.Int32),
                pl.Series("number", [1.1], dtype=pl.Float64),
                pl.Series("string", ["string"], dtype=pl.String),
            ]
        ).lazy()

        save_sqlite_table(source, path=path, fileDialect=DIALECT, overwrite=True)

        target = load_sqlite_table(
            Resource(data=path, fileDialect=DIALECT),
            denormalized=True,
        )
        frame: pl.DataFrame = target.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {
                "boolean": "true",
                "date": "2025-01-01",
                "datetime": "2025-01-01T00:00:00",
                "integer": 1,
                "number": 1.1,
                "string": "string",
            },
        ]
