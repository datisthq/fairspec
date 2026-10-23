from pydantic import TypeAdapter

from fairspec_metadata.models.table_schema import TableSchema

from ..inspect import inspect_model


class TestInspectModel:
    def test_returns_the_parsed_model(self):
        descriptor = {"properties": {"id": {"type": "integer"}}, "x-note": "kept"}
        table_schema, errors = inspect_model(descriptor, model=TypeAdapter(TableSchema))
        assert errors == []
        assert table_schema is not None
        assert table_schema.model_dump(exclude_none=True)["x-note"] == "kept"

    def test_reports_a_mismatch_at_its_pointer(self):
        descriptor = {"properties": {"x": {"type": "integer", "minimum": "low"}}}
        table_schema, errors = inspect_model(
            descriptor,
            model=TypeAdapter(TableSchema),
            root_json_pointer="/resources/0/tableSchema",
        )
        assert table_schema is None
        assert errors
        assert all(error.type == "metadata" for error in errors)
        pointers = [error.jsonPointer for error in errors]
        assert "/resources/0/tableSchema/properties/x/minimum" in pointers
