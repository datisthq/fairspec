from __future__ import annotations

from typing import Annotated

from pydantic import Field

from ..base import FairspecModel

from .common import CreatorNameType


class CreatorNameIdentifier(FairspecModel):
    nameIdentifier: str = Field(
        description="Uniquely identifies an individual or legal entity, according to various schemas"
    )
    nameIdentifierScheme: str = Field(
        description="The name of the name identifier scheme (e.g., ORCID, ISNI, ROR)"
    )
    schemeUri: str | None = Field(
        default=None,
        description="The URI of the name identifier scheme",
    )


class CreatorAffiliation(FairspecModel):
    name: str = Field(
        description="The organizational or institutional affiliation of the creator"
    )
    affiliationIdentifier: str | None = Field(
        default=None,
        description="Uniquely identifies the organizational affiliation of the creator",
    )
    affiliationIdentifierScheme: str | None = Field(
        default=None,
        description="The name of the affiliation identifier scheme",
    )
    schemeUri: str | None = Field(
        default=None,
        description="The URI of the affiliation identifier scheme",
    )


class Creator(FairspecModel):
    name: str = Field(
        description="The main researchers involved in producing the data, or the authors of the publication in priority order"
    )
    nameType: CreatorNameType | None = Field(
        default=None,
        description="The type of name (Organizational or Personal)",
    )
    givenName: str | None = Field(
        default=None,
        description="The personal or first name of the creator",
    )
    familyName: str | None = Field(
        default=None,
        description="The surname or last name of the creator",
    )
    nameIdentifiers: list[CreatorNameIdentifier] | None = Field(
        default=None,
        description="Uniquely identifies an individual or legal entity, according to various schemas",
    )
    affiliation: list[CreatorAffiliation] | None = Field(
        default=None,
        description="The organizational or institutional affiliation of the creator",
    )
    lang: str | None = Field(
        default=None,
        description="Language of the name, specified using ISO 639-1 or ISO 639-3 codes",
    )


Creators = Annotated[
    list[Creator],
    Field(
        min_length=1,
        description="The main researchers involved in producing the data, or the authors of the publication in priority order",
    ),
]
