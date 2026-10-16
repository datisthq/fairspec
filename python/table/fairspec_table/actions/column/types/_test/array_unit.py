from __future__ import annotations

import polars as pl

from fairspec_metadata import ArrayColumn, ArrayColumnProperty
from fairspec_metadata import CellJsonError, CellTypeError

from ..array import inspect_array_column


class TestInspectArrayColumn:
    def test_valid_json_arrays(self):
        table = pl.DataFrame(
            {
                "tags": ['["tag1","tag2"]', "[1,2,3]", '["a","b","c"]'],
            }
        ).lazy()
        column = ArrayColumn(
            name="tags",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_array_column(column, table)

        assert len(errors) == 0

    def test_empty_arrays(self):
        table = pl.DataFrame(
            {
                "items": ["[]", "[]", "[]"],
            }
        ).lazy()
        column = ArrayColumn(
            name="items",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_array_column(column, table)

        assert len(errors) == 0

    def test_null_values(self):
        table = pl.DataFrame(
            {
                "data": ['["value"]', None, "[1,2,3]"],
            }
        ).lazy()
        column = ArrayColumn(
            name="data",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_array_column(column, table)

        assert len(errors) == 0

    def test_json_objects_error(self):
        table = pl.DataFrame(
            {
                "data": ["[1,2,3]", '{"key":"value"}', '["a","b"]'],
            }
        ).lazy()
        column = ArrayColumn(
            name="data",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_array_column(column, table)

        assert len(errors) == 1
        assert isinstance(errors[0], CellTypeError)
        assert errors[0].columnName == "data"
        assert errors[0].columnType == "array"
        assert errors[0].rowNumber == 2
        assert errors[0].cell == '{"key":"value"}'

    def test_invalid_json_error(self):
        table = pl.DataFrame(
            {
                "data": ['["valid"]', "invalid json", "[1,2,3]", "[broken"],
            }
        ).lazy()
        column = ArrayColumn(
            name="data",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_array_column(column, table)

        type_errors = [e for e in errors if e.type == "cell/type"]
        assert len(type_errors) == 2
        assert any(e.rowNumber == 2 and e.cell == "invalid json" for e in type_errors)
        assert any(e.rowNumber == 4 and e.cell == "[broken" for e in type_errors)

    def test_nested_arrays(self):
        table = pl.DataFrame(
            {
                "matrix": [
                    "[[1,2],[3,4]]",
                    "[[5,6],[7,8]]",
                    '[["a","b"],["c","d"]]',
                ],
            }
        ).lazy()
        column = ArrayColumn(
            name="matrix",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_array_column(column, table)

        assert len(errors) == 0

    def test_empty_strings_error(self):
        table = pl.DataFrame(
            {
                "data": ['["valid"]', "", "[1,2,3]"],
            }
        ).lazy()
        column = ArrayColumn(
            name="data",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_array_column(column, table)

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
        column = ArrayColumn(
            name="data",
            type="array",
            property=ArrayColumnProperty(),
        )

        errors = inspect_array_column(column, table)

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

    def test_valid_arrays_matching_json_schema(self):
        table = pl.DataFrame(
            {
                "scores": ["[80,90,100]", "[75,85,95]", "[90,95,100]"],
            }
        ).lazy()
        column = ArrayColumn(
            name="scores",
            type="array",
            property=ArrayColumnProperty(
                items={"type": "number"},
                minItems=3,
                maxItems=3,
            ),
        )

        errors = inspect_array_column(column, table)

        assert len(errors) == 0

    def test_arrays_not_matching_json_schema(self):
        table = pl.DataFrame(
            {
                "numbers": [
                    "[1,2,3]",
                    '["not","numbers"]',
                    "[1]",
                    "[4,5,6]",
                ],
            }
        ).lazy()
        column = ArrayColumn(
            name="numbers",
            type="array",
            property=ArrayColumnProperty(
                items={"type": "number"},
                minItems=2,
            ),
        )

        errors = inspect_array_column(column, table)

        json_errors = [e for e in errors if isinstance(e, CellJsonError)]
        assert len(json_errors) == 3
        assert json_errors[0].rowNumber == 2
        assert json_errors[0].cell == '["not","numbers"]'
        assert json_errors[0].message == "'not' is not of type 'number'"
        assert json_errors[0].jsonPointer == "/0"
        assert json_errors[1].rowNumber == 2
        assert json_errors[1].message == "'numbers' is not of type 'number'"
        assert json_errors[1].jsonPointer == "/1"
        assert json_errors[2].rowNumber == 3
        assert json_errors[2].cell == "[1]"
        assert json_errors[2].message == "[1] is too short"
        assert json_errors[2].jsonPointer == "/"

    def test_complex_json_schema_with_array_of_objects(self):
        table = pl.DataFrame(
            {
                "users": [
                    '[{"name":"John","age":30},{"name":"Jane","age":25}]',
                    '[{"name":"Bob","age":"invalid"}]',
                ],
            }
        ).lazy()
        column = ArrayColumn(
            name="users",
            type="array",
            property=ArrayColumnProperty(
                items={
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "age": {"type": "number"},
                    },
                    "required": ["name", "age"],
                },
            ),
        )

        errors = inspect_array_column(column, table)

        assert len(errors) == 1
        assert isinstance(errors[0], CellJsonError)
        assert errors[0].rowNumber == 2
        assert errors[0].cell == '[{"name":"Bob","age":"invalid"}]'
        assert errors[0].message == "'invalid' is not of type 'number'"
        assert errors[0].jsonPointer == "/0/age"

    def test_json_schema_with_unique_items(self):
        table = pl.DataFrame(
            {
                "tags": [
                    '["unique","values"]',
                    '["duplicate","duplicate"]',
                ],
            }
        ).lazy()
        column = ArrayColumn(
            name="tags",
            type="array",
            property=ArrayColumnProperty(
                items={"type": "string"},
                uniqueItems=True,
            ),
        )

        errors = inspect_array_column(column, table)

        assert len(errors) == 1
        assert isinstance(errors[0], CellJsonError)
        assert errors[0].rowNumber == 2
        assert errors[0].cell == '["duplicate","duplicate"]'
        assert "has non-unique elements" in errors[0].message
        assert errors[0].jsonPointer == "/"
