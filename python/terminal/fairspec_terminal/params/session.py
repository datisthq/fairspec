from typing import Annotated

import typer

Json = Annotated[bool, typer.Option("--json", help="output as JSON")]
Debug = Annotated[
    bool,
    typer.Option(
        "--debug", help="Enable debug mode to print exception details to stderr"
    ),
]
Silent = Annotated[
    bool, typer.Option("--silent", help="suppress all output except errors")
]
