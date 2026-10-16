from typing import Annotated

import typer

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
