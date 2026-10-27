from __future__ import annotations

from pydantic import TypeAdapter

from fairspec_metadata.actions.descriptor.load import load_descriptor
from fairspec_metadata.actions.frictionless.convert import prepare_frictionless_descriptor
from fairspec_metadata.actions.frictionless.mapping import map_frictionless_errors
from fairspec_metadata.actions.descriptor.inspect import inspect_model
from fairspec_metadata.actions.descriptor.validate import validate_descriptor
from fairspec_metadata.actions.profile.load import load_profile
from fairspec_metadata.actions.report.create import create_report
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.file_dialect.file_dialect import FileDialect
from fairspec_metadata.models.profile import ProfileType
from fairspec_metadata.models.report import Report


class FileDialectValidationResult(Report):
    file_dialect: FileDialect | None


def validate_file_dialect(
    source: Descriptor | str,
    *,
    root_json_pointer: str | None = None,
) -> FileDialectValidationResult:
    loaded = load_descriptor(source) if isinstance(source, str) else source

    prepared = prepare_frictionless_descriptor(
        loaded, kind="fileDialect", root_json_pointer=root_json_pointer
    )
    if prepared.descriptor is None:
        return FileDialectValidationResult(
            valid=False, errors=list(prepared.errors), file_dialect=None
        )
    descriptor = prepared.descriptor

    schema = descriptor.get("$schema")
    schema_url = (
        schema
        if isinstance(schema, str)
        else "https://fairspec.org/profiles/latest/file-dialect.json"
    )

    profile = load_profile(schema_url, profile_type=ProfileType.file_dialect)

    report = validate_descriptor(
        descriptor,
        profile=profile,
        root_json_pointer=root_json_pointer,
    )

    file_dialect: FileDialect | None = None
    if report.valid:
        file_dialect, model_errors = inspect_model(
            descriptor,
            model=TypeAdapter(FileDialect),
            root_json_pointer=root_json_pointer,
        )
        report = create_report([*report.errors, *model_errors])

    errors = report.errors
    if prepared.mapping is not None:
        errors = map_frictionless_errors(errors, prepared.mapping)

    return FileDialectValidationResult(
        valid=report.valid,
        errors=errors,
        file_dialect=file_dialect,
    )
