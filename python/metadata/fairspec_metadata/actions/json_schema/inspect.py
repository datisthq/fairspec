from __future__ import annotations

from jsonschema import Draft202012Validator

from fairspec_metadata.models.descriptor import Descriptor


def inspect_json_schema(
    descriptor: Descriptor,
    *,
    root_json_pointer: str | None = None,
) -> list[dict[str, str]]:
    errors: list[dict[str, str]] = []

    validator = Draft202012Validator(Draft202012Validator.META_SCHEMA)
    for error in validator.iter_errors(descriptor):
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
