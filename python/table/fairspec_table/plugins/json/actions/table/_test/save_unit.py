from __future__ import annotations

import json

import polars as pl
from fairspec_dataset import get_temp_file_path
from fairspec_metadata import JsonFileDialect, JsonlFileDialect
from fairspec_metadata.models.file_dialect.common import RowType

from ..load import load_json_table
from ..save import save_json_table

ROW1 = {"id": 1, "name": "english"}
ROW2 = {"id": 2, "name": "中文"}
TABLE = pl.DataFrame([ROW1, ROW2]).lazy()


class TestSaveJsonTable:
    def test_should_save_table_to_file(self):
        path = get_temp_file_path()

        save_json_table(TABLE, path=path, fileDialect=JsonFileDialect())

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == json.dumps([ROW1, ROW2], indent=2, ensure_ascii=False)

    def test_should_handle_property(self):
        path = get_temp_file_path()

        save_json_table(
            TABLE,
            path=path,
            fileDialect=JsonFileDialect(jsonPointer="key"),
        )

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == json.dumps({"key": [ROW1, ROW2]}, indent=2, ensure_ascii=False)

    def test_should_handle_item_keys(self):
        path = get_temp_file_path()

        save_json_table(
            TABLE,
            path=path,
            fileDialect=JsonFileDialect(columnNames=["name"]),
        )

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == json.dumps(
            [{"name": ROW1["name"]}, {"name": ROW2["name"]}],
            indent=2,
            ensure_ascii=False,
        )

    def test_should_handle_item_type_array(self):
        path = get_temp_file_path()

        save_json_table(
            TABLE,
            path=path,
            fileDialect=JsonFileDialect(rowType=RowType.array),
        )

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == json.dumps(
            [list(ROW1.keys()), list(ROW1.values()), list(ROW2.values())],
            indent=2,
            ensure_ascii=False,
        )

    def test_should_save_and_load_various_data_types(self):
        from fairspec_metadata import Resource

        path = get_temp_file_path(format="json")

        source = pl.DataFrame(
            [
                pl.Series("boolean", [True], dtype=pl.Boolean),
                pl.Series("date", ["2025-01-01"], dtype=pl.String),
                pl.Series("integer", [1], dtype=pl.Int32),
                pl.Series("list", [[1.0, 2.0, 3.0]], dtype=pl.List(pl.Float32)),
                pl.Series("number", [1.1], dtype=pl.Float64),
                pl.Series("string", ["string"], dtype=pl.String),
            ]
        ).lazy()

        save_json_table(source, path=path)

        target = load_json_table(Resource(data=path), denormalized=True)
        frame: pl.DataFrame = target.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {
                "boolean": True,
                "date": "2025-01-01",
                "integer": 1,
                "list": [1.0, 2.0, 3.0],
                "number": 1.1,
                "string": "string",
            },
        ]


class TestSaveJsonTableJsonl:
    def test_should_save_table_to_file(self):
        path = get_temp_file_path()

        save_json_table(TABLE, path=path, fileDialect=JsonlFileDialect())

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == "\n".join(
            [
                json.dumps(ROW1, ensure_ascii=False),
                json.dumps(ROW2, ensure_ascii=False),
            ]
        )

    def test_should_handle_item_keys(self):
        path = get_temp_file_path()

        save_json_table(
            TABLE,
            path=path,
            fileDialect=JsonlFileDialect(columnNames=["name"]),
        )

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == "\n".join(
            [
                json.dumps({"name": ROW1["name"]}, ensure_ascii=False),
                json.dumps({"name": ROW2["name"]}, ensure_ascii=False),
            ]
        )

    def test_should_handle_item_type_array(self):
        path = get_temp_file_path()

        save_json_table(
            TABLE,
            path=path,
            fileDialect=JsonlFileDialect(rowType=RowType.array),
        )

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == "\n".join(
            [
                json.dumps(list(ROW1.keys()), ensure_ascii=False),
                json.dumps(list(ROW1.values()), ensure_ascii=False),
                json.dumps(list(ROW2.values()), ensure_ascii=False),
            ]
        )

    def test_should_handle_item_type_object(self):
        path = get_temp_file_path()

        save_json_table(
            TABLE,
            path=path,
            fileDialect=JsonlFileDialect(rowType=RowType.object),
        )

        with open(path, encoding="utf-8") as f:
            content = f.read()
        assert content == "\n".join(
            [
                json.dumps(ROW1, ensure_ascii=False),
                json.dumps(ROW2, ensure_ascii=False),
            ]
        )
