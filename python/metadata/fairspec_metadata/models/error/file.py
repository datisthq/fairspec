from __future__ import annotations

from typing import Annotated, Literal, Union

from pydantic import Field

from .base import BaseError


class TextualError(BaseError):
    type: Literal["file/textual"] = Field(description="Error type identifier")
    actualEncoding: str | None = Field(
        default=None,
        description="The actual encoding format found",
    )


class IntegrityError(BaseError):
    type: Literal["file/integrity"] = Field(description="Error type identifier")
    hashType: str = Field(description="The type of hash algorithm used")
    expectedHash: str = Field(description="The expected hash value")
    actualHash: str = Field(description="The actual hash value found")


FileError = Annotated[
    Union[TextualError, IntegrityError],
    Field(discriminator="type"),
]
