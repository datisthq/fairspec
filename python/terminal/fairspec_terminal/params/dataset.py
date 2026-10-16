from typing import Annotated

import typer

WithRemote = Annotated[
    bool, typer.Option("--with-remote", help="include remote resources")
]
FromDataset = Annotated[
    str | None, typer.Option("-d", "--dataset", help="dataset to select resource from")
]
FromResource = Annotated[
    str | None, typer.Option("-r", "--resource", help="resource in provided dataset")
]
ToFolder = Annotated[str, typer.Option("--to-folder", help="a local output folder path")]
ToArchive = Annotated[
    str, typer.Option("--to-archive", help="a local output zip file path")
]
