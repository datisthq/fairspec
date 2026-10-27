from __future__ import annotations

from ..inspect import inspect_data_schema


class TestInspectDataSchema:
    def test_accepts_the_2020_12_dialect_and_a_fairspec_profile(self):
        for schema in [
            "https://json-schema.org/draft/2020-12/schema",
            "https://fairspec.org/profiles/1.0.0/data-schema.json",
        ]:
            assert inspect_data_schema({"$schema": schema}) == []

    def test_reports_another_dialect_at_schema(self):
        errors = inspect_data_schema(
            {"$schema": "http://json-schema.org/draft-07/schema#"},
            root_json_pointer="/dataSchema",
        )
        assert [error["jsonPointer"] for error in errors] == ["/dataSchema/$schema"]

    def test_accepts_references_that_resolve_within_the_document(self):
        errors = inspect_data_schema(
            {
                "$defs": {"a/b": {"type": "string"}, "c d": {"$anchor": "cd"}},
                "items": [
                    {"$ref": "#/$defs/a~1b"},
                    {"$ref": "#/$defs/c%20d"},
                    {"$ref": "#cd"},
                ],
                "not": {"$ref": "#"},
            }
        )
        assert errors == []

    def test_reports_every_reference_that_leaves_the_document_in_order(self):
        errors = inspect_data_schema(
            {
                "properties": {
                    "a": {"$ref": "https://example.com/schema.json"},
                    "b": {"$ref": "#/$defs/missing"},
                },
                "items": {"$ref": "#/$defs/%ZZ"},
                "const": {"$ref": "https://example.com/data-not-schema"},
            }
        )
        assert [error["jsonPointer"] for error in errors] == [
            "/properties/a/$ref",
            "/properties/b/$ref",
            "/items/$ref",
        ]

    def test_reports_an_index_beyond_any_int(self):
        errors = inspect_data_schema(
            {"$defs": {"a": [{}]}, "not": {"$ref": "#/$defs/a/" + "9" * 5000}}
        )
        assert [error["jsonPointer"] for error in errors] == ["/not/$ref"]
