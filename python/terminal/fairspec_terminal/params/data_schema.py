from typing import Annotated

import typer

DataSchemaPath = Annotated[
    str, typer.Option("--schema", help="path to a data schema descriptor (JSON Schema)")
]
