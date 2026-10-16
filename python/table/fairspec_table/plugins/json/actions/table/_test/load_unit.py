from __future__ import annotations

import polars as pl
from fairspec_dataset import write_temp_file
from fairspec_metadata import JsonFileDialect, JsonlFileDialect, Resource
from fairspec_metadata.models.file_dialect.common import RowType

from ..load import load_json_table


class TestLoadJsonTable:
    def test_should_load_local_file(self):
        body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
        path = write_temp_file(body)

        table = load_json_table(Resource(data=path, fileDialect=JsonFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_load_local_file_multipart(self):
        body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
        path1 = write_temp_file(body)
        path2 = write_temp_file(body)

        table = load_json_table(
            Resource(data=[path1, path2], fileDialect=JsonFileDialect())
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_handle_property(self):
        body = '{"key": [{"id":1,"name":"english"},{"id":2,"name":"中文"}]}'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonFileDialect(jsonPointer="key"))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_handle_item_keys(self):
        body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonFileDialect(columnNames=["name"]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"name": "english"},
            {"name": "中文"},
        ]

    def test_should_handle_item_type_array(self):
        body = '[["id","name"],[1,"english"],[2,"中文"]]'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonFileDialect(rowType=RowType.array))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_load_item_type_object(self):
        body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonFileDialect(rowType=RowType.object))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]


class TestLoadJsonTableJsonl:
    def test_should_load_local_file(self):
        body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
        path = write_temp_file(body)

        table = load_json_table(Resource(data=path, fileDialect=JsonlFileDialect()))
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_load_local_file_multipart(self):
        body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
        path1 = write_temp_file(body)
        path2 = write_temp_file(body)

        table = load_json_table(
            Resource(data=[path1, path2], fileDialect=JsonlFileDialect())
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_handle_item_keys(self):
        body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonlFileDialect(columnNames=["name"]))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"name": "english"},
            {"name": "中文"},
        ]

    def test_should_handle_item_type_array(self):
        body = '["id","name"]\n[1,"english"]\n[2,"中文"]'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonlFileDialect(rowType=RowType.array))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_load_item_type_object(self):
        body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonlFileDialect(rowType=RowType.object))
        )
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]
