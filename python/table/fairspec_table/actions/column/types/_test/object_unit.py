from __future__ import annotations

import polars as pl

from fairspec_metadata import ObjectColumn, ObjectColumnProperty
from fairspec_metadata import CellJsonError, CellTypeError

from ..object import inspect_object_column


class TestInspectObjectColumn:
    def test_valid_json_objects(self):
        table = pl.DataFrame(
            {
                "metadata": [
                    '{"key":"value"}',
                    '{"num":123}',
                    '{"arr":[1,2,3]}',
                ],
            }
        ).lazy()
        column = ObjectColumn(
            name="metadata",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_object_column(column, table)

        assert len(errors) == 0

    def test_json_arrays_error(self):
        table = pl.DataFrame(
            {
                "data": ["[1,2,3]", '{"key":"value"}', '["a","b","c"]'],
            }
        ).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_object_column(column, table)

        assert len(errors) == 2
        assert isinstance(errors[0], CellTypeError)
        assert errors[0].columnName == "data"
        assert errors[0].columnType == "object"
        assert errors[0].rowNumber == 1
        assert errors[0].cell == "[1,2,3]"
        assert errors[1].rowNumber == 3
        assert errors[1].cell == '["a","b","c"]'

    def test_null_values(self):
        table = pl.DataFrame(
            {
                "config": ['{"key":"value"}', None, '{"num":123}'],
            }
        ).lazy()
        column = ObjectColumn(
            name="config",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_object_column(column, table)

        assert len(errors) == 0

    def test_invalid_json_error(self):
        table = pl.DataFrame(
            {
                "data": [
                    '{"valid":true}',
                    "invalid json",
                    '{"key":"value"}',
                    "{broken}",
                ],
            }
        ).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_object_column(column, table)

        type_errors = [e for e in errors if e.type == "cell/type"]
        assert len(type_errors) == 2
        assert any(e.rowNumber == 2 and e.cell == "invalid json" for e in type_errors)
        assert any(e.rowNumber == 4 and e.cell == "{broken}" for e in type_errors)

    def test_complex_nested_json(self):
        table = pl.DataFrame(
            {
                "complex": [
                    '{"user":{"name":"John","age":30,"tags":["admin","user"]}}',
                    '{"nested":{"deep":{"value":true}}}',
                    '{"array":[{"id":1},{"id":2}]}',
                ],
            }
        ).lazy()
        column = ObjectColumn(
            name="complex",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_object_column(column, table)

        assert len(errors) == 0

    def test_empty_strings_error(self):
        table = pl.DataFrame(
            {
                "data": ['{"valid":true}', "", '{"key":"value"}'],
            }
        ).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_object_column(column, table)

        assert len(errors) == 1
        assert errors[0].type == "cell/type"
        assert errors[0].rowNumber == 2
        assert errors[0].cell == ""

    def test_json_primitives_error(self):
        table = pl.DataFrame(
            {
                "data": ['"string"', "123", "true", "false", "null"],
            }
        ).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(),
        )

        errors = inspect_object_column(column, table)

        assert len(errors) == 5
        assert errors[0].rowNumber == 1
        assert errors[0].cell == '"string"'
        assert errors[1].rowNumber == 2
        assert errors[1].cell == "123"
        assert errors[2].rowNumber == 3
        assert errors[2].cell == "true"
        assert errors[3].rowNumber == 4
        assert errors[3].cell == "false"
        assert errors[4].rowNumber == 5
        assert errors[4].cell == "null"

    def test_valid_objects_matching_json_schema(self):
        table = pl.DataFrame(
            {
                "user": [
                    '{"name":"John","age":30}',
                    '{"name":"Jane","age":25}',
                    '{"name":"Bob","age":35}',
                ],
            }
        ).lazy()
        column = ObjectColumn(
            name="user",
            type="object",
            property=ObjectColumnProperty(
                properties={
                    "name": {"type": "string"},
                    "age": {"type": "number"},
                },
                required=["name", "age"],
            ),
        )

        errors = inspect_object_column(column, table)

        assert len(errors) == 0

    def test_objects_not_matching_json_schema(self):
        table = pl.DataFrame(
            {
                "user": [
                    '{"name":"John","age":30}',
                    '{"name":"Jane"}',
                    '{"age":25}',
                    '{"name":"Bob","age":"invalid"}',
                ],
            }
        ).lazy()
        column = ObjectColumn(
            name="user",
            type="object",
            property=ObjectColumnProperty(
                properties={
                    "name": {"type": "string"},
                    "age": {"type": "number"},
                },
                required=["name", "age"],
            ),
        )

        errors = inspect_object_column(column, table)

        json_errors = [e for e in errors if isinstance(e, CellJsonError)]
        assert len(json_errors) == 3
        assert json_errors[0].rowNumber == 2
        assert json_errors[0].cell == '{"name":"Jane"}'
        assert json_errors[0].message == "'age' is a required property"
        assert json_errors[0].jsonPointer == "/"
        assert json_errors[1].rowNumber == 3
        assert json_errors[1].cell == '{"age":25}'
        assert json_errors[1].message == "'name' is a required property"
        assert json_errors[1].jsonPointer == "/"
        assert json_errors[2].rowNumber == 4
        assert json_errors[2].cell == '{"name":"Bob","age":"invalid"}'
        assert json_errors[2].message == "'invalid' is not of type 'number'"
        assert json_errors[2].jsonPointer == "/age"

    def test_complex_json_schema_with_nested_objects(self):
        table = pl.DataFrame(
            {
                "config": [
                    '{"database":{"host":"localhost","port":5432},"cache":{"enabled":true}}',
                    '{"database":{"host":"localhost","port":"invalid"},"cache":{"enabled":true}}',
                ],
            }
        ).lazy()
        column = ObjectColumn(
            name="config",
            type="object",
            property=ObjectColumnProperty(
                properties={
                    "database": {
                        "type": "object",
                        "properties": {
                            "host": {"type": "string"},
                            "port": {"type": "number"},
                        },
                        "required": ["host", "port"],
                    },
                    "cache": {
                        "type": "object",
                        "properties": {
                            "enabled": {"type": "boolean"},
                        },
                        "required": ["enabled"],
                    },
                },
                required=["database", "cache"],
            ),
        )

        errors = inspect_object_column(column, table)

        assert len(errors) == 1
        assert isinstance(errors[0], CellJsonError)
        assert errors[0].rowNumber == 2
        assert errors[0].message == "'invalid' is not of type 'number'"
        assert errors[0].jsonPointer == "/database/port"

    def test_json_schema_with_array_properties(self):
        table = pl.DataFrame(
            {
                "data": [
                    '{"items":[1,2,3],"name":"test"}',
                    '{"items":["not","numbers"],"name":"test"}',
                ],
            }
        ).lazy()
        column = ObjectColumn(
            name="data",
            type="object",
            property=ObjectColumnProperty(
                properties={
                    "items": {
                        "type": "array",
                        "items": {"type": "number"},
                    },
                    "name": {"type": "string"},
                },
                required=["items", "name"],
            ),
        )

        errors = inspect_object_column(column, table)

        json_errors = [e for e in errors if isinstance(e, CellJsonError)]
        assert len(json_errors) == 2
        assert json_errors[0].rowNumber == 2
        assert json_errors[0].message == "'not' is not of type 'number'"
        assert json_errors[0].jsonPointer == "/items/0"
        assert json_errors[1].rowNumber == 2
        assert json_errors[1].message == "'numbers' is not of type 'number'"
        assert json_errors[1].jsonPointer == "/items/1"
