from typing import Annotated

import typer
from fairspec_library import TableCheck

Query = Annotated[
    str | None,
    typer.Argument(
        help="a SQL query to execute against a table (use `self` to refer to the table)"
    ),
]
Overwrite = Annotated[
    bool,
    typer.Option("--overwrite", help="whether to overwrite a file if it already exists"),
]


def parse_table_check(value: str) -> TableCheck:
    name, separator, condition = value.partition("=")
    if not name or not separator:
        raise typer.BadParameter('Expected "name=condition"')
    return {"name": name, "condition": condition}


Check = Annotated[
    list[TableCheck] | None,
    typer.Option(
        "--check",
        parser=parse_table_check,
        metavar="CHECK",
        help="a named SQL condition every row must satisfy, as name=condition (repeatable)",
    ),
]
