from __future__ import annotations

from fairspec_metadata.actions.descriptor.save import save_descriptor
from fairspec_metadata.models.json_schema import JsonSchema


def save_json_schema(
    json_schema: JsonSchema,
    *,
    path: str,
    overwrite: bool = False,
) -> None:
    save_descriptor(json_schema, path=path, overwrite=overwrite)
