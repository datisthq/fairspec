from __future__ import annotations

from fairspec_metadata.actions.json_schema.regex import is_valid_ecma_regex
from fairspec_metadata.models.column.boolean import BooleanColumnProperty
from fairspec_metadata.models.descriptor import Descriptor
from fairspec_metadata.models.error.metadata import MetadataError
from fairspec_metadata.models.table_schema import TableSchema

from .foreign_key import inspect_foreign_key_types

NULLABLE_KEYWORDS = ("enum", "const", "default", "examples")
DEFAULT_TRUE_VALUES = ("true", "True", "TRUE", "1")
DEFAULT_FALSE_VALUES = ("false", "False", "FALSE", "0")


def inspect_table_schema(
    table_schema: TableSchema, *, root_json_pointer: str | None = None
) -> list[MetadataError]:
    """Check the required columns, patterns and key definitions of a table schema against its columns"""
    root = "" if root_json_pointer in (None, "/") else root_json_pointer
    names = set((table_schema.properties or {}).keys())
    errors: list[MetadataError] = []

    def check_columns(columns: list[str], pointer: str) -> None:
        for index, column in enumerate(columns):
            if column not in names:
                errors.append(
                    MetadataError(
                        type="metadata",
                        message=f'column "{column}" is not in properties',
                        jsonPointer=f"{root}{pointer}/{index}",
                    )
                )

    check_columns(table_schema.required or [], "/required")
    for name, property in (table_schema.properties or {}).items():
        pattern = getattr(property, "pattern", None)
        if not isinstance(pattern, str):
            continue
        if not is_valid_ecma_regex(pattern):
            errors.append(
                MetadataError(
                    type="metadata",
                    message=f'pattern "{pattern}" is not a valid regular expression',
                    jsonPointer=f"{root}/properties/{_escape_json_pointer(name)}/pattern",
                )
            )
    for name, property in (table_schema.properties or {}).items():
        if isinstance(property, BooleanColumnProperty):
            errors.extend(_inspect_boolean_values(name, property, root))
    check_columns(table_schema.primaryKey or [], "/primaryKey")
    for index, unique_key in enumerate(table_schema.uniqueKeys or []):
        check_columns(unique_key, f"/uniqueKeys/{index}")
    for index, foreign_key in enumerate(table_schema.foreignKeys or []):
        check_columns(foreign_key.columns, f"/foreignKeys/{index}/columns")
        if len(foreign_key.columns) != len(foreign_key.reference.columns):
            errors.append(
                MetadataError(
                    type="metadata",
                    message="foreign key columns and reference columns differ in length",
                    jsonPointer=f"{root}/foreignKeys/{index}/reference/columns",
                )
            )
        if foreign_key.reference.resource is None:
            errors.extend(
                inspect_foreign_key_types(
                    foreign_key,
                    table_schema=table_schema,
                    referenced_schema=table_schema,
                    json_pointer=f"{root}/foreignKeys/{index}",
                )
            )

    return errors


def inspect_table_schema_nulls(
    descriptor: Descriptor, *, root_json_pointer: str | None = None
) -> list[MetadataError]:
    """Check that enum, const, default and examples hold null only where the column type allows it"""
    root = "" if root_json_pointer in (None, "/") else root_json_pointer
    errors: list[MetadataError] = []
    properties = descriptor.get("properties")
    if not isinstance(properties, dict):
        return errors

    for name, property in properties.items():
        if not isinstance(property, dict) or property.get("type") is None:
            continue
        type = property["type"]
        if isinstance(type, list) and "null" in type:
            continue

        pointer = f"{root}/properties/{_escape_json_pointer(name)}"
        message = (
            f'null is not allowed: the type of column "{name}" does not include "null"'
        )
        for keyword in NULLABLE_KEYWORDS:
            if keyword not in property:
                continue
            value = property[keyword]
            if keyword in ("enum", "examples"):
                if not isinstance(value, list):
                    continue
                for index, item in enumerate(value):
                    if item is None:
                        errors.append(
                            MetadataError(
                                type="metadata",
                                message=message,
                                jsonPointer=f"{pointer}/{keyword}/{index}",
                            )
                        )
            elif value is None:
                errors.append(
                    MetadataError(
                        type="metadata",
                        message=message,
                        jsonPointer=f"{pointer}/{keyword}",
                    )
                )

    return errors


def _inspect_boolean_values(
    name: str, property: BooleanColumnProperty, root: str
) -> list[MetadataError]:
    true_values = property.trueValues
    false_values = property.falseValues
    if true_values is None and false_values is None:
        return []

    # The error lands on the declared list, the false values when both are declared
    keyword = "falseValues" if false_values is not None else "trueValues"
    values = (false_values if false_values is not None else true_values) or []
    others = set(
        (true_values if true_values is not None else DEFAULT_TRUE_VALUES)
        if false_values is not None
        else DEFAULT_FALSE_VALUES
    )

    pointer = f"{root}/properties/{_escape_json_pointer(name)}/{keyword}"
    return [
        MetadataError(
            type="metadata",
            message=f'value "{value}" is in both trueValues and falseValues',
            jsonPointer=f"{pointer}/{index}",
        )
        for index, value in enumerate(values)
        if value in others
    ]


def _escape_json_pointer(key: str) -> str:
    return key.replace("~", "~0").replace("/", "~1")
