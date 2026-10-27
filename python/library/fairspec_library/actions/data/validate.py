from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import (
    DataError,
    FairspecError,
    MetadataError,
    Report,
    ResourceTypeError,
    create_report,
    get_data_paths,
    inspect_json,
    resolve_data_schema,
    validate_data_schema,
)

from .format import DATA_SCHEMA_FORMATS
from .load import load_data

if TYPE_CHECKING:
    from fairspec_metadata import Resource


def validate_data(resource: Resource) -> Report:
    errors: list[FairspecError] = []

    data_schema = resolve_data_schema(resource.dataSchema)
    if not data_schema:
        return create_report()

    if len(get_data_paths(resource)) > 1:
        return create_report(
            [
                MetadataError(
                    type="metadata",
                    message="a data schema needs single-part data",
                    jsonPointer="/dataSchema",
                )
            ]
        )

    schema_report = validate_data_schema(data_schema, root_json_pointer="/dataSchema")
    if not schema_report.valid:
        return create_report(list(schema_report.errors))

    data = load_data(resource)
    if data is None:
        return create_report(
            [ResourceTypeError(type="resource/type", expectedResourceType="data")]
        )

    notes = inspect_json(data, json_schema=data_schema, formats=DATA_SCHEMA_FORMATS)
    for note in notes:
        errors.append(
            DataError(
                type="data",
                message=note["message"],
                jsonPointer=note["jsonPointer"],
            )
        )

    return create_report(errors)
