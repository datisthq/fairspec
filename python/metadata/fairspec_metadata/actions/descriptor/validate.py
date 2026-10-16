from __future__ import annotations

from fairspec_metadata.actions.json.inspect import inspect_json
from fairspec_metadata.actions.report.create import create_report
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.error.metadata import MetadataError
from fairspec_metadata.models.profile import Profile
from fairspec_metadata.models.report import Report


def validate_descriptor(
    descriptor: Descriptor,
    *,
    profile: Profile,
    root_json_pointer: str | None = None,
) -> Report:
    errors = inspect_json(
        descriptor,
        json_schema=profile,
        root_json_pointer=root_json_pointer,
    )
    return create_report(
        [
            MetadataError(
                type="metadata",
                message=error["message"],
                jsonPointer=error["jsonPointer"],
            )
            for error in errors
        ]
    )
