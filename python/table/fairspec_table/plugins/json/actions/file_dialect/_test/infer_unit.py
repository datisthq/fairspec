from __future__ import annotations

from fairspec_dataset import write_temp_file
from fairspec_metadata import (
    CsvFileDialect,
    JsonFileDialect,
    JsonlFileDialect,
    Resource,
)
from fairspec_metadata.models.file_dialect.common import RowType

from ..infer import infer_json_file_dialect


class TestInferJsonFileDialectJsonArrayOfObjects:
    def test_should_detect_row_type_as_object_without_header_rows(self):
        body = '[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]'
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonFileDialect())

        result = infer_json_file_dialect(resource)

        assert result == JsonFileDialect(rowType=RowType.object)


class TestInferJsonFileDialectJsonArrayOfArrays:
    def test_should_detect_headers_when_first_row_is_header(self):
        body = '[["id", "name"], [1, "Alice"], [2, "Bob"]]'
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonFileDialect())

        result = infer_json_file_dialect(resource)

        assert result == JsonFileDialect(rowType=RowType.array, headerRows=[1])

    def test_should_detect_no_headers_when_data_rows_only(self):
        body = "[[1, 2, 3], [4, 5, 6], [7, 8, 9]]"
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonFileDialect())

        result = infer_json_file_dialect(resource)

        assert result == JsonFileDialect(rowType=RowType.array, headerRows=False)


class TestInferJsonFileDialectNestedJson:
    def test_should_detect_json_pointer_for_nested_data_structure(self):
        body = '{"metadata": {"version": "1.0"}, "data": [["id", "name"], [1, "Alice"], [2, "Bob"]]}'
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonFileDialect())

        result = infer_json_file_dialect(resource)

        assert isinstance(result, JsonFileDialect)
        assert result.jsonPointer == "data"
        assert result.rowType == RowType.array

    def test_should_detect_json_pointer_with_array_of_objects(self):
        body = '{"metadata": {"version": "1.0"}, "records": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}'
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonFileDialect())

        result = infer_json_file_dialect(resource)

        assert result == JsonFileDialect(jsonPointer="records", rowType=RowType.object)


class TestInferJsonFileDialectJsonl:
    def test_should_detect_row_type_for_array_jsonl(self):
        body = '[1, "Alice", 30]\n[2, "Bob", 25]\n[3, "Charlie", 35]'
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonlFileDialect())

        result = infer_json_file_dialect(resource)

        assert result == JsonlFileDialect(rowType=RowType.array, headerRows=False)

    def test_should_detect_row_type_for_object_jsonl(self):
        body = '{"id": 1, "name": "Alice"}\n{"id": 2, "name": "Bob"}\n{"id": 3, "name": "Charlie"}'
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonlFileDialect())

        result = infer_json_file_dialect(resource)

        assert result == JsonlFileDialect(rowType=RowType.object)


class TestInferJsonFileDialectEdgeCases:
    def test_should_return_format_only_for_empty_array(self):
        body = "[]"
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonFileDialect())

        result = infer_json_file_dialect(resource)

        assert result == JsonFileDialect()

    def test_should_return_format_only_for_invalid_json(self):
        body = "{this is not valid json"
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonFileDialect())

        result = infer_json_file_dialect(resource)

        assert result == JsonFileDialect()

    def test_should_return_none_for_inline_data(self):
        resource = Resource(
            data=[
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"},
            ],
            fileDialect=JsonFileDialect(),
        )

        result = infer_json_file_dialect(resource)

        assert result is None

    def test_should_return_none_for_unsupported_format(self):
        body = "id,name\n1,Alice\n2,Bob"
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=CsvFileDialect())

        result = infer_json_file_dialect(resource)

        assert result is None

    def test_should_return_format_only_for_single_row_array(self):
        body = "[[1, 2, 3]]"
        path = write_temp_file(body)

        resource = Resource(data=path, fileDialect=JsonFileDialect())

        result = infer_json_file_dialect(resource)

        assert result == JsonFileDialect(rowType=RowType.array, headerRows=False)

    def test_should_handle_non_file_path_errors_gracefully(self):
        resource = Resource(
            data="/nonexistent/path/to/file.json",
            fileDialect=JsonFileDialect(),
        )

        result = infer_json_file_dialect(resource)

        assert result == JsonFileDialect()
