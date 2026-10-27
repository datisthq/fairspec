from fairspec_metadata.models.table_schema import TableSchema

from ..inspect import inspect_table_schema, inspect_table_schema_nulls


class TestInspectTableSchema:
    def test_required_column_not_in_properties(self):
        table_schema = TableSchema.model_validate(
            {"required": ["id", "nope"], "properties": {"id": {"type": "integer"}}}
        )
        errors = inspect_table_schema(table_schema)
        assert [error.model_dump(exclude_none=True) for error in errors] == [
            {
                "type": "metadata",
                "message": 'column "nope" is not in properties',
                "jsonPointer": "/required/1",
            }
        ]

    def test_pattern_not_a_regular_expression(self):
        table_schema = TableSchema.model_validate(
            {"properties": {"a/b": {"type": "string", "pattern": "[a-"}}}
        )
        errors = inspect_table_schema(table_schema)
        assert [error.model_dump(exclude_none=True) for error in errors] == [
            {
                "type": "metadata",
                "message": 'pattern "[a-" is not a valid regular expression',
                "jsonPointer": "/properties/a~1b/pattern",
            }
        ]

    def test_valid_pattern(self):
        table_schema = TableSchema.model_validate(
            {"properties": {"code": {"type": "string", "pattern": "^[A-Z]{3}-[0-9]{4}$"}}}
        )
        assert inspect_table_schema(table_schema) == []

    def test_self_referencing_foreign_key_with_differing_types(self):
        table_schema = TableSchema.model_validate(
            {
                "properties": {
                    "id": {"type": "string"},
                    "parent_id": {"type": "integer"},
                    "rank": {"type": "number"},
                },
                "foreignKeys": [
                    {"columns": ["parent_id"], "reference": {"columns": ["rank"]}},
                    {"columns": ["parent_id"], "reference": {"columns": ["id"]}},
                ],
            }
        )
        errors = inspect_table_schema(table_schema)
        assert [error.model_dump(exclude_none=True) for error in errors] == [
            {
                "type": "metadata",
                "message": 'foreign key column "parent_id" is integer, but "id" is string',
                "jsonPointer": "/foreignKeys/1",
            }
        ]

    def test_boolean_value_in_both_lists(self):
        table_schema = TableSchema.model_validate(
            {
                "properties": {
                    "x": {
                        "type": "boolean",
                        "trueValues": ["yes", "no"],
                        "falseValues": ["no"],
                    },
                    "y": {"type": "boolean", "trueValues": ["yes", "0"]},
                    "z": {"type": "boolean", "falseValues": ["Yes", "true"]},
                }
            }
        )
        errors = inspect_table_schema(table_schema)
        assert [error.model_dump(exclude_none=True) for error in errors] == [
            {
                "type": "metadata",
                "message": 'value "no" is in both trueValues and falseValues',
                "jsonPointer": "/properties/x/falseValues/0",
            },
            {
                "type": "metadata",
                "message": 'value "0" is in both trueValues and falseValues',
                "jsonPointer": "/properties/y/trueValues/1",
            },
            {
                "type": "metadata",
                "message": 'value "true" is in both trueValues and falseValues',
                "jsonPointer": "/properties/z/falseValues/1",
            },
        ]

    def test_boolean_values_in_separate_lists(self):
        table_schema = TableSchema.model_validate(
            {
                "properties": {
                    "x": {"type": "boolean", "trueValues": ["1"], "falseValues": ["0"]}
                }
            }
        )
        assert inspect_table_schema(table_schema) == []


class TestInspectTableSchemaNulls:
    def test_null_values_on_column_without_null(self):
        errors = inspect_table_schema_nulls(
            {
                "properties": {
                    "x": {
                        "type": "string",
                        "const": None,
                        "enum": ["a", None],
                        "default": None,
                        "examples": [None],
                    }
                }
            },
            root_json_pointer="/resources/0/tableSchema",
        )
        pointer = "/resources/0/tableSchema/properties/x"
        assert [error.jsonPointer for error in errors] == [
            f"{pointer}/enum/1",
            f"{pointer}/const",
            f"{pointer}/default",
            f"{pointer}/examples/0",
        ]
        assert errors[0].message == (
            'null is not allowed: the type of column "x" does not include "null"'
        )

    def test_null_values_on_nullable_or_unknown_column(self):
        errors = inspect_table_schema_nulls(
            {
                "properties": {
                    "x": {"type": ["string", "null"], "const": None, "enum": ["a", None]},
                    "y": {"const": None},
                }
            }
        )
        assert errors == []
