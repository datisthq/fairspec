from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import (
    DataError,
    FairspecError,
    Report,
    create_report,
    inspect_json,
    resolve_data_schema,
)

from .load import load_data

if TYPE_CHECKING:
    from fairspec_metadata import Resource


def validate_data(resource: Resource) -> Report:
    errors: list[FairspecError] = []

    data_schema = resolve_data_schema(resource.dataSchema)
    if not data_schema:
        return create_report()

    data = load_data(resource)
    if data is None:
        return create_report()

    notes = inspect_json(data, json_schema=data_schema)
    for note in notes:
        errors.append(
            DataError(
                type="data",
                message=note["message"],
                jsonPointer=note["jsonPointer"],
            )
        )

    return create_report(errors)
