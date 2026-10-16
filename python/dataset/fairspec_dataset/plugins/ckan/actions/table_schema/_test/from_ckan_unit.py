from __future__ import annotations

import json
import os

from fairspec_dataset.plugins.ckan.models.field import CkanField
from fairspec_dataset.plugins.ckan.models.schema import CkanSchema
from ..from_ckan import convert_table_schema_from_ckan


def _load_fixture() -> CkanSchema:
    path = os.path.join(os.path.dirname(__file__), "fixtures", "ckan-schema.json")
    with open(path) as f:
        return CkanSchema(**json.load(f))


class TestConvertTableSchemaFromCkan:
    def test_converts_ckan_schema_to_fairspec_table_schema(self):
        ckan_schema = _load_fixture()

        result = convert_table_schema_from_ckan(ckan_schema)

        assert result.properties is not None
        properties = result.properties
        assert ckan_schema.fields is not None
        assert len(properties) == len(ckan_schema.fields)

        id_col = properties["id"]
        assert id_col.type == "integer"
        assert id_col.title == "ID"
        assert id_col.description == "Unique identifier"

        name_col = properties["name"]
        assert name_col.type == "string"
        assert name_col.title == "Name"
        assert name_col.description == "Person's full name"

        age_col = properties["age"]
        assert age_col.type == "integer"
        assert age_col.title is None
        assert age_col.description is None

        score_col = properties["score"]
        assert score_col.type == "number"
        assert score_col.title == "Score"
        assert score_col.description == "Test score"

        is_active_col = properties["is_active"]
        assert is_active_col.type == "boolean"

        birth_date_col = properties["birth_date"]
        assert birth_date_col.type == "string"
        assert birth_date_col.format == "date"
        assert birth_date_col.title == "Birth Date"
        assert birth_date_col.description == "Date of birth"

        start_time_col = properties["start_time"]
        assert start_time_col.type == "string"
        assert start_time_col.format == "time"

        created_at_col = properties["created_at"]
        assert created_at_col.type == "string"
        assert created_at_col.format == "date-time"
        assert created_at_col.title == "Created At"
        assert created_at_col.description == "Timestamp when record was created"

        metadata_col = properties["metadata"]
        assert metadata_col.type == "object"

        tags_col = properties["tags"]
        assert tags_col.type == "array"
        assert tags_col.title == "Tags"
        assert tags_col.description == "List of tags"

    def test_converts_ckan_type_aliases(self):
        ckan_schema = _load_fixture()

        result = convert_table_schema_from_ckan(ckan_schema)
        assert result.properties is not None
        properties = result.properties

        assert properties["string_field"].type == "string"
        assert properties["integer_field"].type == "integer"
        assert properties["number_field"].type == "number"
        assert properties["float_field"].type == "number"
        assert properties["boolean_field"].type == "boolean"
        assert properties["datetime_field"].type == "string"
        assert properties["datetime_field"].format == "date-time"
        assert properties["object_field"].type == "object"

    def test_handles_unknown_field_types(self):
        ckan_schema = _load_fixture()

        result = convert_table_schema_from_ckan(ckan_schema)
        assert result.properties is not None
        properties = result.properties

        assert properties["unknown_field"].type == "string"

    def test_respects_type_override_in_field_info(self):
        ckan_schema = _load_fixture()

        result = convert_table_schema_from_ckan(ckan_schema)
        assert result.properties is not None
        properties = result.properties

        override_col = properties["override_field"]
        assert override_col.type == "integer"
        assert override_col.title == "Override Field"
        assert override_col.description == "Field with type override"

    def test_handles_empty_fields_array(self):
        ckan_schema = CkanSchema(fields=[])

        result = convert_table_schema_from_ckan(ckan_schema)

        assert result.properties is None or len(result.properties) == 0

    def test_handles_fields_without_info_object(self):
        ckan_schema = CkanSchema(fields=[CkanField(id="simple_field", type="text")])

        result = convert_table_schema_from_ckan(ckan_schema)
        assert result.properties is not None
        properties = result.properties

        assert len(properties) == 1
        col = properties["simple_field"]
        assert col.type == "string"
        assert col.title is None
        assert col.description is None

    def test_handles_case_insensitive_type_conversion(self):
        ckan_schema = CkanSchema(
            fields=[
                CkanField(id="field1", type="TEXT"),
                CkanField(id="field2", type="INT"),
                CkanField(id="field3", type="BOOL"),
                CkanField(id="field4", type="TIMESTAMP"),
            ],
        )

        result = convert_table_schema_from_ckan(ckan_schema)
        assert result.properties is not None
        properties = result.properties

        assert properties["field1"].type == "string"
        assert properties["field2"].type == "integer"
        assert properties["field3"].type == "boolean"
        assert properties["field4"].type == "string"
        assert properties["field4"].format == "date-time"
