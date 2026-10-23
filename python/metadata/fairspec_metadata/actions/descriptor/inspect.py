from __future__ import annotations

from typing import TypeVar

from pydantic import TypeAdapter, ValidationError

from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.error.metadata import MetadataError

T = TypeVar("T")


def inspect_model(
    descriptor: Descriptor,
    *,
    model: TypeAdapter[T],
    root_json_pointer: str | None = None,
) -> tuple[T | None, list[MetadataError]]:
    """Parse a descriptor with a built-in model, reporting each mismatch as a metadata error"""
    try:
        return model.validate_python(descriptor), []
    except ValidationError as error:
        root = "" if root_json_pointer in (None, "/") else root_json_pointer
        errors: list[MetadataError] = []
        pointers: set[str] = set()
        for item in error.errors(include_url=False):
            pointer = f"{root}{_get_json_pointer(descriptor, item['loc'])}"
            if pointer in pointers:
                continue
            pointers.add(pointer)
            errors.append(
                MetadataError(type="metadata", message=item["msg"], jsonPointer=pointer)
            )
        return None, errors


def _get_json_pointer(descriptor: Descriptor, loc: tuple[int | str, ...]) -> str:
    value: object = descriptor
    pointer = ""
    for key in loc:
        if isinstance(value, dict) and isinstance(key, str) and key in value:
            value = value[key]
        elif isinstance(value, list) and isinstance(key, int) and key < len(value):
            value = value[key]
        else:
            continue
        pointer += "/" + str(key).replace("~", "~0").replace("/", "~1")
    return pointer
