from __future__ import annotations

import re
from urllib.parse import unquote

from fairspec_metadata.models.descriptor import Descriptor

META_SCHEMA_URL = "https://json-schema.org/draft/2020-12/schema"
PROFILE_REGEX = re.compile(r"https://fairspec\.org/profiles/[^/]+/data-schema\.json")

SCHEMA_MAP_KEYWORDS = [
    "$defs",
    "definitions",
    "properties",
    "patternProperties",
    "dependentSchemas",
]
SCHEMA_KEYWORDS = [
    "additionalProperties",
    "unevaluatedProperties",
    "items",
    "additionalItems",
    "unevaluatedItems",
    "contains",
    "propertyNames",
    "not",
    "if",
    "then",
    "else",
    "contentSchema",
    "allOf",
    "anyOf",
    "oneOf",
    "prefixItems",
]


def inspect_data_schema(
    descriptor: Descriptor, *, root_json_pointer: str | None = None
) -> list[dict[str, str]]:
    """Tell the `$schema` a Data Schema may not name, and every `$ref` that does not
    resolve within the document, which is never fetched"""
    root_path = root_json_pointer or ""
    errors: list[dict[str, str]] = []

    schema = descriptor.get("$schema")
    if (
        isinstance(schema, str)
        and schema != META_SCHEMA_URL
        and not PROFILE_REGEX.fullmatch(schema)
    ):
        errors.append(
            {
                "message": "must be a Fairspec Data Schema profile or the JSON Schema 2020-12 dialect",
                "jsonPointer": f"{root_path}/$schema",
            }
        )
        return errors

    schemas = _list_subschemas(descriptor, "")
    for subschema, pointer in schemas:
        for keyword in ("$ref", "$dynamicRef"):
            ref = subschema.get(keyword)
            if isinstance(ref, str) and not _get_is_local_ref(ref, descriptor, schemas):
                errors.append(
                    {
                        "message": "must resolve within the data schema",
                        "jsonPointer": f"{root_path}{pointer}/{keyword}",
                    }
                )

    return errors


def _list_subschemas(schema: Descriptor, pointer: str) -> list[tuple[Descriptor, str]]:
    result: list[tuple[Descriptor, str]] = [(schema, pointer)]
    for keyword in SCHEMA_MAP_KEYWORDS:
        mapping = schema.get(keyword)
        if not isinstance(mapping, dict):
            continue
        for key, value in mapping.items():
            if isinstance(value, dict):
                token = _escape_token(key)
                result.extend(_list_subschemas(value, f"{pointer}/{keyword}/{token}"))
    for keyword in SCHEMA_KEYWORDS:
        value = schema.get(keyword)
        if isinstance(value, list):
            for index, item in enumerate(value):
                if isinstance(item, dict):
                    result.extend(_list_subschemas(item, f"{pointer}/{keyword}/{index}"))
        elif isinstance(value, dict):
            result.extend(_list_subschemas(value, f"{pointer}/{keyword}"))
    return result


def _get_is_local_ref(
    ref: str, root: Descriptor, schemas: list[tuple[Descriptor, str]]
) -> bool:
    base, _, fragment = ref.partition("#")

    target: Descriptor | None = root
    if base:
        target = next(
            (schema for schema, _ in schemas if schema.get("$id") == base), None
        )
        if target is None:
            return False

    if not fragment:
        return True
    if not fragment.startswith("/"):
        return any(
            schema.get("$anchor") == fragment or schema.get("$dynamicAnchor") == fragment
            for schema, _ in schemas
        )

    if re.search(r"%(?![0-9A-Fa-f]{2})", fragment):
        return False
    try:
        decoded = unquote(fragment, errors="strict")
    except UnicodeDecodeError:
        return False

    value: object = target
    for token in decoded[1:].split("/"):
        key = token.replace("~1", "/").replace("~0", "~")
        if isinstance(value, list):
            # Compared by length first: an unbounded digit string never reaches int()
            is_index = re.fullmatch(r"0|[1-9][0-9]*", key) is not None
            if not is_index or len(key) > len(str(len(value))) or int(key) >= len(value):
                return False
            value = value[int(key)]
        elif isinstance(value, dict) and key in value:
            value = value[key]
        else:
            return False
    return True


def _escape_token(token: str) -> str:
    return token.replace("~", "~0").replace("/", "~1")
