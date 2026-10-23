from __future__ import annotations

from jsonschema import Draft202012Validator
from pydantic import TypeAdapter

from fairspec_metadata.actions.descriptor.load import load_descriptor
from fairspec_metadata.actions.descriptor.inspect import inspect_model
from fairspec_metadata.actions.descriptor.validate import validate_descriptor
from fairspec_metadata.actions.profile.load import load_profile
from fairspec_metadata.actions.report.create import create_report
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.profile import ProfileType
from fairspec_metadata.models.report import Report
from fairspec_metadata.models.table_schema import TableSchema


class TableSchemaValidationResult(Report):
    table_schema: TableSchema | None


def validate_table_schema(
    source: Descriptor | str,
    *,
    root_json_pointer: str | None = None,
) -> TableSchemaValidationResult:
    descriptor = load_descriptor(source) if isinstance(source, str) else source

    schema = descriptor.get("$schema")
    schema_url = (
        schema
        if isinstance(schema, str) and schema != Draft202012Validator.META_SCHEMA["$id"]
        else "https://fairspec.org/profiles/latest/table-schema.json"
    )

    profile = load_profile(schema_url, profile_type=ProfileType.table_schema)

    report = validate_descriptor(
        descriptor,
        profile=profile,
        root_json_pointer=root_json_pointer,
    )

    table_schema: TableSchema | None = None
    if report.valid:
        table_schema, model_errors = inspect_model(
            descriptor,
            model=TypeAdapter(TableSchema),
            root_json_pointer=root_json_pointer,
        )
        report = create_report([*report.errors, *model_errors])

    return TableSchemaValidationResult(
        valid=report.valid,
        errors=report.errors,
        table_schema=table_schema,
    )
