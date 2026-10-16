from ..inspect import inspect_json_schema


class TestInspectJsonSchemaValid:
    def test_valid_empty_object_schema(self):
        errors = inspect_json_schema({"type": "object"})
        assert errors == []

    def test_valid_string_schema(self):
        errors = inspect_json_schema({"type": "string"})
        assert errors == []

    def test_valid_schema_with_properties(self):
        errors = inspect_json_schema(
            {
                "type": "object",
                "properties": {"name": {"type": "string"}},
            }
        )
        assert errors == []

    def test_valid_schema_with_required(self):
        errors = inspect_json_schema(
            {
                "type": "object",
                "required": ["name"],
                "properties": {"name": {"type": "string"}},
            }
        )
        assert errors == []

    def test_valid_array_schema(self):
        errors = inspect_json_schema(
            {
                "type": "array",
                "items": {"type": "integer"},
            }
        )
        assert errors == []

    def test_valid_draft_2020_12_schema(self):
        errors = inspect_json_schema(
            {
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "type": "object",
            }
        )
        assert errors == []


class TestInspectJsonSchemaInvalid:
    def test_invalid_type_value(self):
        errors = inspect_json_schema({"type": "invalid"})
        assert len(errors) > 0
        assert errors[0]["jsonPointer"] == "/type"

    def test_invalid_required_not_array(self):
        errors = inspect_json_schema({"required": "name"})
        assert len(errors) > 0

    def test_invalid_properties_not_object(self):
        errors = inspect_json_schema({"properties": "invalid"})
        assert len(errors) > 0

    def test_root_json_pointer_combined(self):
        errors = inspect_json_schema({"type": "invalid"}, root_json_pointer="/root")
        assert len(errors) > 0
        assert errors[0]["jsonPointer"] == "/root/type"
