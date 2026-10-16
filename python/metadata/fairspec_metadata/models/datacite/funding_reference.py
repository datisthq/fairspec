from __future__ import annotations

from typing import Annotated

from pydantic import Field

from ..base import FairspecModel

from .common import FunderIdentifierType


class FundingReference(FairspecModel):
    funderName: str = Field(description="Name of the funding provider")
    funderIdentifier: str | None = Field(
        default=None,
        description="Uniquely identifies a funding entity, according to various identifier schemes",
    )
    funderIdentifierType: FunderIdentifierType | None = Field(
        default=None,
        description="The type of the funderIdentifier (e.g., ISNI, GRID, Crossref Funder ID, ROR, Other)",
    )
    awardNumber: str | None = Field(
        default=None,
        description="The code assigned by the funder to a sponsored award (grant)",
    )
    awardUri: str | None = Field(
        default=None,
        description="The URI leading to a page provided by the funder for more information about the award (grant)",
    )
    awardTitle: str | None = Field(
        default=None,
        description="The human readable title of the award (grant)",
    )


FundingReferences = Annotated[
    list[FundingReference],
    Field(
        description="Information about financial support (funding) for the resource being registered"
    ),
]
