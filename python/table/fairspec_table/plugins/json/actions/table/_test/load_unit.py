from __future__ import annotations

from unittest.mock import patch

import polars as pl
from fairspec_dataset import write_temp_file
from fairspec_metadata import JsonFileDialect, JsonlFileDialect, Resource
from fairspec_metadata.models.file_dialect.common import RowType

from ..load import load_json_table


class TestLoadJsonTable:
    def test_should_return_none_for_top_level_object(self):
        path = write_temp_file('{"id": 1}')

        table = load_json_table(Resource(data=path, fileDialect=JsonFileDialect()))

        assert table is None

    def test_should_load_local_file(self):
        body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
        path = write_temp_file(body)

        table = load_json_table(Resource(data=path, fileDialect=JsonFileDialect()))
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_read_an_array_of_objects_natively(self):
        path = write_temp_file('[{"id":1},{"id":2}]', format="json")

        with patch.object(pl, "read_json", wraps=pl.read_json) as read_json:
            table = load_json_table(Resource(data=path), denormalized=True)
            assert table is not None

        read_json.assert_called_once()

    def test_should_ignore_a_byte_order_mark(self):
        path = write_temp_file('\ufeff[{"id":1}]', format="json")

        table = load_json_table(Resource(data=path))
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id": 1}]

    def test_should_order_the_columns_by_the_first_row_a_key_is_seen_in(self):
        path = write_temp_file('[{"b":1},{"a":2,"b":3}]', format="json")

        table = load_json_table(Resource(data=path), denormalized=True)
        assert table is not None

        assert table.collect_schema().names() == ["b", "a"]

    def test_should_order_jsonl_columns_by_the_first_line_a_key_is_seen_in(self):
        path = write_temp_file('{"b":1}\n{"a":2,"b":3}\n', format="jsonl")

        table = load_json_table(Resource(data=path), denormalized=True)
        assert table is not None

        assert table.collect_schema().names() == ["b", "a"]

    def test_should_fall_back_when_the_native_reader_refuses_the_rows(self):
        path = write_temp_file('[{"id":1},5,{"id":2}]')

        table = load_json_table(Resource(data=path, fileDialect=JsonFileDialect()))
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id": 1}, {"id": 2}]

    def test_should_load_local_file_multipart(self):
        body = '[{"id":1,"name":"english"},{"id":2,"name":"中文"}]'
        path1 = write_temp_file(body)
        path2 = write_temp_file(body)

        table = load_json_table(
            Resource(data=[path1, path2], fileDialect=JsonFileDialect())
        )
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_handle_nested_json_pointer(self):
        body = '{"data": {"a/b": [{"id":1,"name":"english"}]}}'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonFileDialect(jsonPointer="/data/a~1b"))
        )
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id": 1, "name": "english"}]

    def test_should_take_column_names_for_an_empty_array(self):
        path = write_temp_file('{"items":[]}')

        table = load_json_table(
            Resource(
                data=path,
                fileDialect=JsonFileDialect(
                    jsonPointer="/items",
                    rowType=RowType.object,
                    columnNames=["id", "name"],
                ),
            )
        )
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.columns == ["id", "name"]
        assert frame.height == 0

    def test_should_handle_property(self):
        body = '{"key": [{"id":1,"name":"english"},{"id":2,"name":"中文"}]}'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonFileDialect(jsonPointer="/key"))
        )
        assert table is not None
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
        assert table is not None
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
        assert table is not None
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
        assert table is not None
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
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_scan_an_inferred_object_row_dialect_natively(self):
        path = write_temp_file('{"id":1}\n{"id":2}', format="jsonl")

        with patch.object(pl, "scan_ndjson", wraps=pl.scan_ndjson) as scan_ndjson:
            table = load_json_table(Resource(data=path), denormalized=True)
            assert table is not None
            table.collect()

        scan_ndjson.assert_called_once_with(path, infer_schema_length=None)

    def test_should_load_local_file_multipart(self):
        body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
        path1 = write_temp_file(body)
        path2 = write_temp_file(body)

        table = load_json_table(
            Resource(data=[path1, path2], fileDialect=JsonlFileDialect())
        )
        assert table is not None
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
        assert table is not None
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
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]

    def test_should_read_array_rows_when_row_type_is_absent(self):
        path = write_temp_file('["id","name"]\n[1,"english"]')

        table = load_json_table(Resource(data=path, fileDialect=JsonlFileDialect()))
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [{"id": 1, "name": "english"}]

    def test_should_load_item_type_object(self):
        body = '{"id":1,"name":"english"}\n{"id":2,"name":"中文"}'
        path = write_temp_file(body)

        table = load_json_table(
            Resource(data=path, fileDialect=JsonlFileDialect(rowType=RowType.object))
        )
        assert table is not None
        frame: pl.DataFrame = table.collect()  # ty: ignore[invalid-assignment]

        assert frame.to_dicts() == [
            {"id": 1, "name": "english"},
            {"id": 2, "name": "中文"},
        ]
