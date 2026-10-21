import json
from typing import Annotated, Literal

from fairspec_metadata import Descriptor
from fireargs import Option

from fairspec_terminal.models.input import CommandInput

TableSchemaPath = Annotated[
    str | None,
    Option(
        name="schema",
        alias="schema",
        help_group="Table Schema",
        description="path to a Table Schema descriptor",
    ),
]


def parse_column_types(value: str) -> dict[str, str]:
    result: dict[str, str] = {}
    for pair in value.split(","):
        name, _, type = pair.partition(":")
        if name and type:
            result[name] = type
    return result


def parse_column_properties(value: str) -> dict[str, Descriptor]:
    result = json.loads(value)
    if not isinstance(result, dict):
        raise ValueError("--column-properties must be a JSON object")
    return result


def parse_list(value: str) -> list[str]:
    return value.split(",")


class TableSchemaParams(CommandInput):
    column_types: Annotated[
        dict[str, str] | None,
        Option(
            help_group="Table Schema",
            parse=parse_column_types,
            description="a list of comma-separated column name:type pairs to use for the schema",
        ),
    ] = None
    column_properties: Annotated[
        dict[str, Descriptor] | None,
        Option(
            help_group="Table Schema",
            parse=parse_column_properties,
            description="a JSON object of column name to column property to use for the schema",
        ),
    ] = None
    missing_values: Annotated[
        list[str] | None,
        Option(
            help_group="Table Schema",
            parse=parse_list,
            description="comma-separated values to treat as missing",
        ),
    ] = None
    decimal_char: Annotated[
        str | None,
        Option(
            help_group="Table Schema", description="character to use as decimal separator"
        ),
    ] = None
    group_char: Annotated[
        str | None,
        Option(
            help_group="Table Schema", description="character to use for digit grouping"
        ),
    ] = None
    true_values: Annotated[
        list[str] | None,
        Option(
            help_group="Table Schema",
            parse=parse_list,
            description="values to treat as true",
        ),
    ] = None
    false_values: Annotated[
        list[str] | None,
        Option(
            help_group="Table Schema",
            parse=parse_list,
            description="values to treat as false",
        ),
    ] = None
    datetime_format: Annotated[
        str | None,
        Option(help_group="Table Schema", description="datetime format pattern"),
    ] = None
    date_format: Annotated[
        str | None, Option(help_group="Table Schema", description="date format pattern")
    ] = None
    time_format: Annotated[
        str | None, Option(help_group="Table Schema", description="time format pattern")
    ] = None
    array_type: Annotated[
        Literal["array", "list"] | None,
        Option(help_group="Table Schema", description="array type (array or list)"),
    ] = None
    list_delimiter: Annotated[
        str | None,
        Option(help_group="Table Schema", description="delimiter for list values"),
    ] = None
    list_item_type: Annotated[
        Literal[
            "string",
            "number",
            "boolean",
            "date",
            "date-time",
            "local-date-time",
            "integer",
            "time",
            "local-time",
        ]
        | None,
        Option(help_group="Table Schema", description="type of items in lists"),
    ] = None
    sample_rows: Annotated[
        int | None,
        Option(
            help_group="Table Schema",
            description="number of rows to sample for schema inference",
        ),
    ] = None
    confidence: Annotated[
        float | None,
        Option(
            help_group="Table Schema",
            description="confidence threshold for schema inference",
        ),
    ] = None
    comma_decimal: Annotated[
        bool,
        Option(
            help_group="Table Schema",
            description="use comma as decimal separator in schema inference",
        ),
    ] = False
    month_first: Annotated[
        bool,
        Option(
            help_group="Table Schema",
            description="interpret dates as month-first in schema inference",
        ),
    ] = False
    keep_strings: Annotated[
        bool,
        Option(
            help_group="Table Schema",
            description="keep fields as strings instead of inferring types",
        ),
    ] = False
