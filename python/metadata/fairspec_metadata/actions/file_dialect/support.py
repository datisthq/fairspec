from __future__ import annotations

from typing import TYPE_CHECKING

from pydantic import TypeAdapter

from fairspec_metadata.actions.resource.data import get_data_path
from fairspec_metadata.models.file_dialect.file_dialect import FileDialect

from .infer import infer_file_dialect_format
from .resolve import resolve_file_dialect

if TYPE_CHECKING:
    from fairspec_metadata.models.resource import Resource

_file_dialect_adapter = TypeAdapter(FileDialect)


def get_supported_file_dialect(
    resource: Resource, supported_formats: list[str]
) -> FileDialect | None:
    data_path = get_data_path(resource)
    if not data_path:
        return None

    resolved = resolve_file_dialect(resource.fileDialect)
    if resolved is None:
        format = infer_file_dialect_format(resource)
        if format:
            resolved = _file_dialect_adapter.validate_python({"format": format})

    if resolved is None:
        return None

    format_value = getattr(resolved, "format", None)

    for supported_format in supported_formats:
        if format_value == supported_format:
            return resolved

    return None
