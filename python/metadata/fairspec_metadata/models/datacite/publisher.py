from __future__ import annotations

from pydantic import Field

from ..base import FairspecModel


class Publisher(FairspecModel):
    name: str = Field(
        description="The name of the entity that holds, archives, publishes, prints, distributes, releases, issues, or produces the resource"
    )
    publisherIdentifier: str | None = Field(
        default=None,
        description="Uniquely identifies the publisher, according to various identifier schemes",
    )
    publisherIdentifierScheme: str | None = Field(
        default=None,
        description="The name of the publisher identifier scheme (e.g., ISNI, ROR, Crossref Funder ID)",
    )
    schemeUri: str | None = Field(
        default=None,
        description="The URI of the publisher identifier scheme",
    )
    lang: str | None = Field(
        default=None,
        description="Language of the publisher name, specified using ISO 639-1 or ISO 639-3 codes",
    )
