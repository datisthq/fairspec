from __future__ import annotations

from typing import TYPE_CHECKING

from genson import SchemaBuilder

from fairspec_library.actions.data.load import load_data

if TYPE_CHECKING:
    from fairspec_metadata import JsonSchema, Resource


def infer_data_schema(resource: Resource) -> JsonSchema | None:
    data = load_data(resource)
    if not data:
        return None

    try:
        builder = SchemaBuilder()
        builder.add_object(data)
        return builder.to_schema()
    except Exception:
        return None
