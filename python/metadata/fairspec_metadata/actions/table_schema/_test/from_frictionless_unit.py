from ....models.frictionless.schema import FrictionlessSchema
from ..from_frictionless import convert_table_schema_from_frictionless


def convert(descriptor: dict) -> dict:
    return convert_table_schema_from_frictionless(
        FrictionlessSchema.model_validate(descriptor)
    )


class TestConvertTableSchemaFromFrictionless:
    def test_allows_null_unless_a_field_is_required(self):
        table_schema = convert(
            {
                "fields": [
                    {"name": "id", "type": "integer", "constraints": {"required": True}},
                    {"name": "name", "type": "string"},
                ]
            }
        )

        assert table_schema["properties"] == {
            "id": {"type": "integer"},
            "name": {"type": ["string", "null"]},
        }

    def test_drops_the_default_missing_values(self):
        assert "missingValues" not in convert({"fields": [], "missingValues": [""]})
        assert convert({"fields": [], "missingValues": ["", "NA"]})["missingValues"] == [
            "",
            "NA",
        ]

    def test_reads_a_datetime_pattern_without_an_offset_as_local(self):
        table_schema = convert(
            {
                "fields": [
                    {"name": "a", "type": "datetime"},
                    {"name": "b", "type": "datetime", "format": "%Y-%m-%d %H:%M"},
                    {"name": "c", "type": "datetime", "format": "%Y-%m-%dT%H:%M%z"},
                ]
            }
        )

        assert table_schema["properties"] == {
            "a": {"type": ["string", "null"], "format": "date-time"},
            "b": {
                "type": ["string", "null"],
                "format": "local-date-time",
                "temporalFormat": "%Y-%m-%d %H:%M",
            },
            "c": {
                "type": ["string", "null"],
                "format": "date-time",
                "temporalFormat": "%Y-%m-%dT%H:%M%z",
            },
        }

    def test_renames_referenced_resources_and_resolves_a_self_reference(self):
        table_schema = convert(
            {
                "fields": [{"name": "id"}, {"name": "parent"}],
                "foreignKeys": [
                    {"fields": "parent", "reference": {"resource": "", "fields": "id"}},
                    {
                        "fields": ["id"],
                        "reference": {"resource": "other-table", "fields": ["id"]},
                    },
                ],
            }
        )

        assert table_schema["foreignKeys"] == [
            {"columns": ["parent"], "reference": {"columns": ["id"]}},
            {
                "columns": ["id"],
                "reference": {"resource": "other_table", "columns": ["id"]},
            },
        ]
