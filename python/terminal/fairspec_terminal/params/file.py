from typing import Annotated, Literal

from pydantic import Field

HashType = Annotated[
    Literal["md5", "sha1", "sha256", "sha512"], Field(description="hash type")
]
SampleBytes = Annotated[int | None, Field(description="sample size in bytes")]
Hash = Annotated[
    str | None,
    Field(description="expected file hash calculated with the specified hash type"),
]
Lines = Annotated[int | None, Field(description="maximum number of lines")]
