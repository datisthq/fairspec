from ..inspect import inspect_json


SCHEMA = {
    "type": "object",
    "properties": {
        "name": {"type": "string"},
        "age": {"type": "integer"},
        "address": {
            "type": "object",
            "properties": {
                "city": {"type": "string"},
            },
            "required": ["city"],
        },
    },
    "required": ["name"],
}


class TestInspectJson:
    def test_valid_value(self):
        errors = inspect_json({"name": "test"}, json_schema=SCHEMA)
        assert errors == []

    def test_type_mismatch(self):
        errors = inspect_json({"name": 123}, json_schema=SCHEMA)
        assert len(errors) == 1
        assert errors[0]["jsonPointer"] == "/name"

    def test_missing_required(self):
        errors = inspect_json({}, json_schema=SCHEMA)
        assert len(errors) == 1
        assert "name" in errors[0]["message"]
        assert errors[0]["jsonPointer"] == ""

    def test_escapes_reference_tokens(self):
        errors = inspect_json(
            {"a/b": "x", "c~d": "x"},
            json_schema={
                "properties": {"a/b": {"type": "integer"}, "c~d": {"type": "integer"}}
            },
        )
        assert sorted(error["jsonPointer"] for error in errors) == ["/a~1b", "/c~0d"]

    def test_nested_validation(self):
        errors = inspect_json(
            {"name": "test", "address": {}},
            json_schema=SCHEMA,
        )
        assert len(errors) == 1
        assert errors[0]["jsonPointer"] == "/address"

    def test_multiple_errors(self):
        errors = inspect_json(
            {"name": 123, "age": "not_a_number"},
            json_schema=SCHEMA,
        )
        assert len(errors) == 2
