from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_library.actions.data.load import load_data

if TYPE_CHECKING:
    from fairspec_metadata import JsonSchema, Resource


def infer_data_schema(resource: Resource) -> JsonSchema | None:
    data = load_data(resource)
    if data is None:
        return None

    try:
        return _infer_value_schema(data)
    except TypeError:
        return None


def _infer_value_schema(value: object) -> JsonSchema:
    if value is None:
        return {"type": "null"}
    if isinstance(value, bool):
        return {"type": "boolean"}
    if isinstance(value, int):
        return {"type": "integer"}
    if isinstance(value, float):
        return {"type": "integer" if value.is_integer() else "number"}
    if isinstance(value, str):
        return {"type": "string"}
    if isinstance(value, list):
        items = [_infer_value_schema(item) for item in value]
        if items and all(item == items[0] for item in items):
            return {"type": "array", "items": items[0]}
        return {"type": "array"}
    if isinstance(value, dict):
        if not value:
            return {"type": "object"}
        return {
            "type": "object",
            "properties": {
                str(key): _infer_value_schema(item) for key, item in value.items()
            },
        }
    raise TypeError(f"Invalid JSON value: {value!r}")
