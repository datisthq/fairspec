from __future__ import annotations

from pydantic import BaseModel, JsonValue, ValidationError

from fairspec_metadata import FairspecException, MetadataError, create_report


def assert_frictionless_model[T: BaseModel](
    descriptor: JsonValue, *, model: type[T], json_pointer: str = ""
) -> T:
    """Parse a Frictionless descriptor, raising its mismatches as a metadata report"""
    try:
        return model.model_validate(descriptor)
    except ValidationError as exception:
        pointers: dict[str, str] = {}
        for error in exception.errors():
            is_missing = error["type"] == "missing"
            pointer = json_pointer + _create_pointer(
                descriptor, error["loc"], is_missing=is_missing
            )
            pointers.setdefault(pointer, error["msg"])

    raise FairspecException(
        "Invalid Frictionless descriptor",
        report=create_report(
            [
                MetadataError(type="metadata", message=message, jsonPointer=pointer)
                for pointer, message in pointers.items()
            ]
        ),
    )


# Pydantic tags each union member in the location, zod reports the union itself
def _create_pointer(
    descriptor: JsonValue, loc: tuple[int | str, ...], *, is_missing: bool
) -> str:
    pointer = ""
    value = descriptor
    for index, key in enumerate(loc):
        if isinstance(value, dict) and isinstance(key, str):
            if key in value:
                value = value[key]
            elif is_missing and index == len(loc) - 1:
                value = None
            else:
                break
        elif isinstance(value, list) and isinstance(key, int) and key < len(value):
            value = value[key]
        else:
            break
        pointer += "/" + str(key).replace("~", "~0").replace("/", "~1")
    return pointer
