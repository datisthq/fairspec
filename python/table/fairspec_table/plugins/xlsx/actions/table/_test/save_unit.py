from __future__ import annotations

import polars as pl
from fairspec_dataset import get_temp_file_path
from fairspec_metadata import OdsFileDialect, Resource, XlsxFileDialect

from ..load import load_xlsx_table
from ..save import save_xlsx_table
from .test_data import read_test_data

ROW1 = {"id": 1, "name": "english"}
ROW2 = {"id": 2, "name": "中文"}
TABLE = pl.DataFrame([ROW1, ROW2]).lazy()


class TestSaveXlsxTableXlsx:
    def test_should_save_table_to_file(self):
        path = get_temp_file_path()

        save_xlsx_table(TABLE, path=path, fileDialect=XlsxFileDialect())

        data = read_test_data(path)
        assert data == [ROW1, ROW2]

    def test_should_save_and_load_various_data_types(self):
        path = get_temp_file_path(format="xlsx")

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

        save_xlsx_table(source, path=path, fileDialect=XlsxFileDialect())

        target = load_xlsx_table(
            Resource(data=path, fileDialect=XlsxFileDialect()),
            denormalized=True,
        )
        frame: pl.DataFrame = target.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {
                "boolean": True,
                "date": "2025-01-01",
                "datetime": "2025-01-01T00:00:00",
                "integer": 1,
                "number": 1.1,
                "string": "string",
            }
        ]


class TestSaveXlsxTableOds:
    def test_should_save_table_to_file(self):
        path = get_temp_file_path()

        save_xlsx_table(TABLE, path=path, fileDialect=OdsFileDialect())

        data = read_test_data(path)
        assert data == [ROW1, ROW2]

    def test_should_save_and_load_various_data_types(self):
        path = get_temp_file_path(format="ods")

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

        save_xlsx_table(source, path=path, fileDialect=OdsFileDialect())

        target = load_xlsx_table(
            Resource(data=path, fileDialect=OdsFileDialect()),
            denormalized=True,
        )
        frame: pl.DataFrame = target.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {
                "boolean": True,
                "date": "2025-01-01",
                "datetime": "2025-01-01T00:00:00",
                "integer": 1,
                "number": 1.1,
                "string": "string",
            }
        ]
