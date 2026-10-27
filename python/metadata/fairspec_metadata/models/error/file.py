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


class HeaderError(BaseError):
    type: Literal["file/header"] = Field(description="Error type identifier")
    partNumber: int = Field(
        description="The 1-based position of the part among the paths of the resource"
    )
    columnNames: list[str] = Field(
        description="The column names the part's header gives, empty when it has none"
    )


FileError = Annotated[
    Union[TextualError, IntegrityError, HeaderError],
    Field(discriminator="type"),
]
