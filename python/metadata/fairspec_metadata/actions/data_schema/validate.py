from __future__ import annotations

from fairspec_metadata.actions.descriptor.load import load_descriptor
from fairspec_metadata.actions.descriptor.validate import validate_descriptor
from fairspec_metadata.actions.profile.load import load_profile
from fairspec_metadata.models.data_schema import DataSchema
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.profile import ProfileType
from fairspec_metadata.models.report import Report


class DataSchemaValidationResult(Report):
    data_schema: DataSchema | None


def validate_data_schema(
    source: Descriptor | str,
    *,
    root_json_pointer: str | None = None,
) -> DataSchemaValidationResult:
    descriptor = load_descriptor(source) if isinstance(source, str) else source

    schema = descriptor.get("$schema")
    schema_url = (
        schema
        if isinstance(schema, str)
        else "https://fairspec.org/profiles/latest/data-schema.json"
    )

    profile = load_profile(schema_url, profile_type=ProfileType.data_schema)

    report = validate_descriptor(
        descriptor,
        profile=profile,
        root_json_pointer=root_json_pointer,
    )

    data_schema: DataSchema | None = None
    if report.valid:
        data_schema = descriptor

    return DataSchemaValidationResult(
        valid=report.valid,
        errors=report.errors,
        data_schema=data_schema,
    )
