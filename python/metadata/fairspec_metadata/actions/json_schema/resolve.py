from __future__ import annotations

from .load import load_json_schema
from fairspec_metadata.models.json_schema import JsonSchema


def resolve_json_schema(
    json_schema: JsonSchema | str | None = None,
) -> JsonSchema | None:
    if json_schema is None:
        return None

    if isinstance(json_schema, str):
        return load_json_schema(json_schema)

    return json_schema
