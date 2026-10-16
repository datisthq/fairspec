from __future__ import annotations

from typing import Any

import referencing
from jsonschema import Draft202012Validator

from fairspec_metadata.actions.json_schema.load import load_json_schema
from fairspec_metadata.models.json_schema import JsonSchema


def inspect_json(
    value: Any,
    *,
    json_schema: JsonSchema | str,
    root_json_pointer: str | None = None,
) -> list[dict[str, str]]:
    if isinstance(json_schema, str):
        json_schema = load_json_schema(json_schema)

    registry = referencing.Registry(retrieve=_retrieve)  # type: ignore[call-arg]
    validator = Draft202012Validator(json_schema, registry=registry, format_checker=None)

    errors: list[dict[str, str]] = []
    for error in validator.iter_errors(value):
        instance_path = _deque_to_json_pointer(error.absolute_path)
        root_path = root_json_pointer or ""
        json_pointer = _combine_json_pointers(root_path, instance_path)

        errors.append(
            {
                "message": error.message,
                "jsonPointer": json_pointer,
            }
        )

    return errors


def _retrieve(uri: str) -> referencing.Resource:
    schema = load_json_schema(uri, only_remote=True)
    return referencing.Resource.from_contents(schema)


def _deque_to_json_pointer(path: object) -> str:
    parts = list(path)  # type: ignore[arg-type]
    if not parts:
        return "/"
    return "/" + "/".join(str(p) for p in parts)


def _combine_json_pointers(root: str, instance: str) -> str:
    if root == "" or root == "/":
        return instance
    if instance == "/":
        return root
    return root + instance
