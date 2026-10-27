from fairspec_metadata.models.error.cell import CellTypeError
from fairspec_metadata.models.error.metadata import MetadataError

from ..convert import convert_descriptor_from_frictionless
from ..mapping import map_frictionless_errors

MAPPING = convert_descriptor_from_frictionless(
    {
        "resources": [
            {
                "name": "fruit-list",
                "path": "fruit.csv",
                "schema": {
                    "fields": [
                        {"name": "id", "type": "integer"},
                        {"name": "day", "type": "date", "format": "%d/%m/%Y"},
                    ],
                    "missingValues": ["", "NA"],
                },
            }
        ]
    },
    kind="dataset",
).mapping


def create_error(json_pointer: str) -> MetadataError:
    return MetadataError(type="metadata", message="", jsonPointer=json_pointer)


class TestMapFrictionlessErrors:
    def test_points_at_the_field_and_key_a_property_came_from(self):
        errors = map_frictionless_errors(
            [
                create_error("/resources/0/tableSchema/properties/day/temporalFormat"),
                create_error("/resources/0/tableSchema/properties/day"),
            ],
            MAPPING,
        )

        assert [getattr(error, "jsonPointer") for error in errors] == [
            "/resources/0/schema/fields/1/format",
            "/resources/0/schema/fields/1",
        ]

    def test_keeps_the_key_path_below_a_value_copied_as_it_is(self):
        [error] = map_frictionless_errors(
            [create_error("/resources/0/tableSchema/missingValues/1")], MAPPING
        )

        assert getattr(error, "jsonPointer") == "/resources/0/schema/missingValues/1"

    def test_names_resources_as_the_package_does(self):
        [error] = map_frictionless_errors(
            [
                CellTypeError.model_validate(
                    {
                        "type": "cell/type",
                        "columnName": "id",
                        "columnType": "integer",
                        "rowNumber": 1,
                        "cell": "x",
                        "resourceName": "fruit_list",
                    }
                )
            ],
            MAPPING,
        )

        assert error.resourceName == "fruit-list"
