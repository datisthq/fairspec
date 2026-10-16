from __future__ import annotations

import polars as pl
from fairspec_dataset import get_temp_file_path
from fairspec_metadata import CsvFileDialect, Resource, TsvFileDialect

from ..load import load_csv_table
from ..save import save_csv_table

ROW1 = {"id": 1.0, "name": "Alice"}
ROW2 = {"id": 2.0, "name": "Bob"}
ROW3 = {"id": 3.0, "name": "Charlie"}
TABLE = pl.DataFrame([ROW1, ROW2, ROW3]).lazy()


class TestSaveCsvTable:
    def test_should_save_table_to_file(self):
        path = get_temp_file_path()

        save_csv_table(TABLE, path=path, fileDialect=CsvFileDialect())

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == "id,name\n1.0,Alice\n2.0,Bob\n3.0,Charlie\n"

    def test_should_save_with_custom_delimiter(self):
        path = get_temp_file_path()

        save_csv_table(
            TABLE,
            path=path,
            fileDialect=CsvFileDialect(delimiter=";"),
        )

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == "id;name\n1.0;Alice\n2.0;Bob\n3.0;Charlie\n"

    def test_should_save_without_header(self):
        path = get_temp_file_path()

        save_csv_table(
            TABLE,
            path=path,
            fileDialect=CsvFileDialect(headerRows=False),
        )

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == "1.0,Alice\n2.0,Bob\n3.0,Charlie\n"

    def test_should_save_with_custom_quote_char(self):
        path = get_temp_file_path()

        table = pl.DataFrame(
            {
                "id": [1.0, 2.0, 3.0],
                "name": ["Alice,Smith", "Bob,Jones", "Charlie,Brown"],
            }
        ).lazy()

        save_csv_table(
            table,
            path=path,
            fileDialect=CsvFileDialect(quoteChar="'"),
        )

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == (
            "id,name\n1.0,'Alice,Smith'\n2.0,'Bob,Jones'\n3.0,'Charlie,Brown'\n"
        )

    def test_should_save_and_load_various_data_types(self):
        path = get_temp_file_path(format="csv")

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

        save_csv_table(source, path=path)

        target = load_csv_table(Resource(data=path), denormalized=True)
        frame: pl.DataFrame = target.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {
                "boolean": "true",
                "date": "2025-01-01",
                "datetime": "2025-01-01T00:00:00",
                "integer": "1",
                "number": "1.1",
                "string": "string",
            },
        ]


class TestSaveCsvTableTsv:
    def test_should_save_table_to_file(self):
        path = get_temp_file_path()

        save_csv_table(TABLE, path=path, fileDialect=TsvFileDialect())

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == "id\tname\n1.0\tAlice\n2.0\tBob\n3.0\tCharlie\n"
