from typing import Annotated

import typer

HashType = Annotated[str, typer.Option("--hash-type", help="hash type")]
Bytes_ = Annotated[
    str | None, typer.Option("--bytes", help="expected file size in bytes")
]
SampleBytes = Annotated[
    int | None, typer.Option("--sample-bytes", help="sample size in bytes")
]
Hash_ = Annotated[
    str | None,
    typer.Option(
        "--hash", help="expected file hash calculated with the specified hash type"
    ),
]
