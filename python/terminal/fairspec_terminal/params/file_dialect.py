from typing import Annotated, Literal

from fireargs import Option

from fairspec_terminal.models.input import CommandInput


def parse_header_rows(value: str) -> Literal[False] | list[int]:
    return False if value == "false" else parse_row_numbers(value)


def parse_row_numbers(value: str) -> list[int]:
    return [int(item) for item in value.split(",")]


class DialectParams(CommandInput):
    dialect: Annotated[
        str | None,
        Option(
            help_group="Dialect",
            description="path to a Dialect descriptor, Fairspec or Frictionless",
        ),
    ] = None
    format: Annotated[
        str | None,
        Option(help_group="Dialect", description="format type (csv, json, xlsx, etc)"),
    ] = None
    delimiter: Annotated[
        str | None,
        Option(
            help_group="Dialect",
            description="character used to separate fields in the data",
        ),
    ] = None
    line_terminator: Annotated[
        str | None,
        Option(
            help_group="Dialect", description="character sequence used to terminate rows"
        ),
    ] = None
    quote_char: Annotated[
        str | None,
        Option(help_group="Dialect", description="character used to quote fields"),
    ] = None
    null_sequence: Annotated[
        str | None,
        Option(
            help_group="Dialect",
            description="character sequence representing null or missing values",
        ),
    ] = None
    header_rows: Annotated[
        Literal[False] | list[int] | None,
        Option(
            help_group="Dialect",
            parse=parse_header_rows,
            description="comma-separated row numbers (1-indexed) for headers, or 'false' to disable",
        ),
    ] = None
    header_join: Annotated[
        str | None,
        Option(
            help_group="Dialect", description="character used to join multi-line headers"
        ),
    ] = None
    comment_rows: Annotated[
        list[int] | None,
        Option(
            help_group="Dialect",
            parse=parse_row_numbers,
            description="comma-separated row numbers (1-indexed) to exclude from data",
        ),
    ] = None
    comment_prefix: Annotated[
        str | None,
        Option(
            help_group="Dialect",
            description="character sequence denoting the start of a comment line",
        ),
    ] = None
    column_names: Annotated[
        list[str] | None,
        Option(
            help_group="Dialect",
            parse=lambda value: value.split(","),
            description="comma-separated list of column names",
        ),
    ] = None
    json_pointer: Annotated[
        str | None,
        Option(
            help_group="Dialect",
            description="JSON pointer to the data array within the JSON document",
        ),
    ] = None
    row_type: Annotated[
        Literal["array", "object"] | None,
        Option(help_group="Dialect", description="the type of each row in the data"),
    ] = None
    sheet_number: Annotated[
        int | None,
        Option(
            help_group="Dialect",
            description="for spreadsheet data, the sheet number to read (0-indexed)",
        ),
    ] = None
    sheet_name: Annotated[
        str | None,
        Option(
            help_group="Dialect",
            description="for spreadsheet data, the sheet name to read",
        ),
    ] = None
    table_name: Annotated[
        str | None,
        Option(
            help_group="Dialect",
            description="for database sources, the table name to read",
        ),
    ] = None
