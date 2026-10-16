from __future__ import annotations

from typing import Annotated

from pydantic import Field

from ..base import FairspecModel


class Rights(FairspecModel):
    rights: str | None = Field(
        default=None,
        description="Any rights information for this resource",
    )
    rightsUri: str | None = Field(
        default=None,
        description="The URI of the license",
    )
    rightsIdentifier: str | None = Field(
        default=None,
        description="A short, standardized version of the license name",
    )
    rightsIdentifierScheme: str | None = Field(
        default=None,
        description="The name of the scheme (e.g., SPDX)",
    )
    schemeUri: str | None = Field(
        default=None,
        description="The URI of the rightsIdentifierScheme",
    )
    lang: str | None = Field(
        default=None,
        description="Language of the rights statement, specified using ISO 639-1 or ISO 639-3 codes",
    )


RightsList = Annotated[
    list[Rights],
    Field(description="Any rights information for this resource"),
]
