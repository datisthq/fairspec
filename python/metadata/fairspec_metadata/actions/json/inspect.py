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

    validator = _get_validator(json_schema)

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


# Keyed by identity, holding the schema so its id is never reused: callers pass one
# schema object for a whole column, and a schema mutated after use keeps its validator
_VALIDATORS: dict[int, tuple[JsonSchema, Draft202012Validator]] = {}


def _get_validator(json_schema: JsonSchema) -> Draft202012Validator:
    cached = _VALIDATORS.get(id(json_schema))
    if cached and cached[0] is json_schema:
        return cached[1]

    registry = referencing.Registry(retrieve=_retrieve)  # type: ignore[call-arg]
    validator = Draft202012Validator(json_schema, registry=registry, format_checker=None)
    if len(_VALIDATORS) >= 128:
        _VALIDATORS.clear()
    _VALIDATORS[id(json_schema)] = (json_schema, validator)
    return validator


def _retrieve(uri: str) -> referencing.Resource:
    schema = load_json_schema(uri, only_remote=True)
    return referencing.Resource.from_contents(schema)


def _deque_to_json_pointer(path: object) -> str:
    parts = list(path)  # type: ignore[arg-type]
    return "".join("/" + str(p).replace("~", "~0").replace("/", "~1") for p in parts)


def _combine_json_pointers(root: str, instance: str) -> str:
    if root == "/":
        return instance
    return root + instance
