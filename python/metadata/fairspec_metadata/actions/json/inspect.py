from __future__ import annotations

from collections.abc import Callable
from typing import Any

import referencing
from jsonschema import Draft202012Validator, FormatChecker
from jsonschema.protocols import Validator

from fairspec_metadata.actions.json_schema.load import load_json_schema
from fairspec_metadata.models.json_schema import JsonSchema


def inspect_json(
    value: Any,
    *,
    json_schema: JsonSchema | str,
    root_json_pointer: str | None = None,
    formats: dict[str, Callable[[str], bool]] | None = None,
) -> list[dict[str, str]]:
    if isinstance(json_schema, str):
        json_schema = load_json_schema(json_schema)

    validator = _get_validator(json_schema, formats)

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
_VALIDATORS: dict[tuple[int, int], tuple[JsonSchema, Validator]] = {}


def _get_validator(
    json_schema: JsonSchema, formats: dict[str, Callable[[str], bool]] | None
) -> Validator:
    key = (id(json_schema), id(formats))
    cached = _VALIDATORS.get(key)
    if cached and cached[0] is json_schema:
        return cached[1]

    # A schema checked with formats is data, whose references never leave the document
    registry = (
        referencing.Registry()
        if formats is not None
        else referencing.Registry(retrieve=_retrieve)  # type: ignore[call-arg]
    )
    validator = Draft202012Validator(
        _replace_false_subschemas(json_schema),
        registry=registry,
        format_checker=_create_format_checker(formats) if formats is not None else None,
    )
    if len(_VALIDATORS) >= 128:
        _VALIDATORS.clear()
    _VALIDATORS[key] = (json_schema, validator)
    return validator


def _create_format_checker(formats: dict[str, Callable[[str], bool]]) -> FormatChecker:
    checker = FormatChecker(formats=())
    for name, check in formats.items():
        checker.checks(name)(
            lambda value, check=check: not isinstance(value, str) or check(value)
        )
    return checker


# jsonschema drops the instance path of an error a `false` subschema raises, so
# `false` under a keyword that descends into a member becomes the equivalent `not: {}`
def _replace_false_subschemas(schema: object) -> object:
    if not isinstance(schema, dict):
        return schema

    result = dict(schema)
    for keyword, value in schema.items():
        if keyword in ("properties", "patternProperties", "$defs", "dependentSchemas"):
            if isinstance(value, dict):
                result[keyword] = {
                    key: _replace_false(item, keyword != "$defs")
                    for key, item in value.items()
                }
        elif keyword in ("prefixItems", "allOf", "anyOf", "oneOf"):
            if isinstance(value, list):
                result[keyword] = [
                    _replace_false(item, keyword == "prefixItems") for item in value
                ]
        elif keyword in _SCHEMA_KEYWORDS:
            result[keyword] = _replace_false(value, False)
    return result


def _replace_false(schema: object, is_member: bool) -> object:
    if schema is False and is_member:
        return {"not": {}}
    return _replace_false_subschemas(schema)


_SCHEMA_KEYWORDS = {
    "additionalProperties",
    "unevaluatedProperties",
    "items",
    "unevaluatedItems",
    "contains",
    "propertyNames",
    "not",
    "if",
    "then",
    "else",
    "contentSchema",
}


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
