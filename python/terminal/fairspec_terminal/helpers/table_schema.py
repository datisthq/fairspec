from __future__ import annotations

import json
from typing import TYPE_CHECKING, Literal

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor
    from fairspec_table.models.schema import InferTableSchemaOptions

ARRAY_TYPES: tuple[Literal["array", "list"], ...] = ("array", "list")
LIST_ITEM_TYPES: tuple[
    Literal[
        "string",
        "integer",
        "number",
        "boolean",
        "date",
        "time",
        "local-time",
        "date-time",
        "local-date-time",
    ],
    ...,
] = (
    "string",
    "integer",
    "number",
    "boolean",
    "date",
    "time",
    "local-time",
    "date-time",
    "local-date-time",
)


def create_infer_table_schema_options(
    *,
    sample_rows: int | None = None,
    confidence: float | None = None,
    comma_decimal: bool = False,
    month_first: bool = False,
    keep_strings: bool = False,
    column_types: str | None = None,
    column_properties: str | None = None,
    missing_values: str | None = None,
    decimal_char: str | None = None,
    group_char: str | None = None,
    true_values: str | None = None,
    false_values: str | None = None,
    datetime_format: str | None = None,
    date_format: str | None = None,
    time_format: str | None = None,
    array_type: str | None = None,
    list_delimiter: str | None = None,
    list_item_type: str | None = None,
) -> InferTableSchemaOptions:
    options: InferTableSchemaOptions = {}

    if sample_rows is not None:
        options["sampleRows"] = sample_rows
    if confidence is not None:
        options["confidence"] = confidence
    if comma_decimal:
        options["commaDecimal"] = True
    if month_first:
        options["monthFirst"] = True
    if keep_strings:
        options["keepStrings"] = True
    if column_types:
        options["columnTypes"] = _parse_column_types(column_types)
    if column_properties:
        options["columnProperties"] = _parse_column_properties(column_properties)
    if missing_values:
        options["missingValues"] = missing_values.split(",")
    if decimal_char:
        options["decimalChar"] = decimal_char
    if group_char:
        options["groupChar"] = group_char
    if true_values:
        options["trueValues"] = true_values.split(",")
    if false_values:
        options["falseValues"] = false_values.split(",")
    if datetime_format:
        options["datetimeFormat"] = datetime_format
    if date_format:
        options["dateFormat"] = date_format
    if time_format:
        options["timeFormat"] = time_format
    for item in ARRAY_TYPES:
        if item == array_type:
            options["arrayType"] = item
    if list_delimiter:
        options["listDelimiter"] = list_delimiter
    for item in LIST_ITEM_TYPES:
        if item == list_item_type:
            options["listItemType"] = item

    return options


def _parse_column_types(value: str) -> dict[str, str]:
    result: dict[str, str] = {}
    for pair in value.split(","):
        name, _, type = pair.partition(":")
        if name and type:
            result[name] = type
    return result


def _parse_column_properties(value: str) -> dict[str, Descriptor]:
    result = json.loads(value)
    if not isinstance(result, dict):
        raise ValueError("--column-properties must be a JSON object")
    return result
