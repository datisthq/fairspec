from typing import Annotated

import typer

OptionalPath = Annotated[str | None, typer.Argument(help="local or remote path")]
RequiredPath = Annotated[str, typer.Argument(help="local or remote path")]
RequiredFilePath = Annotated[str, typer.Argument(help="local or remote path to the file")]
VariadicPaths = Annotated[list[str], typer.Argument(help="local paths to files")]
ToPath = Annotated[str | None, typer.Option("--to-path", help="a local output path")]
ToPathRequired = Annotated[str, typer.Option("--to-path", help="a local output path")]
