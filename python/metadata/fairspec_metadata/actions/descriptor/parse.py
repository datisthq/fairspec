from __future__ import annotations

import json

from fairspec_metadata.models.descriptor import Descriptor


class Error(Exception):
    pass


def parse_descriptor(text: str) -> Descriptor:
    value = json.loads(text)
    if not isinstance(value, dict):
        raise Error(
            f"Invalid descriptor: expected a JSON object, got {_get_json_type(value)}"
        )
    return value


def _get_json_type(value: object) -> str:
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, (int, float)):
        return "number"
    if isinstance(value, str):
        return "string"
    return "array"
