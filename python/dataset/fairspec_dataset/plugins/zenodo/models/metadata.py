from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .creator import ZenodoCreator


class ZenodoRelatedIdentifier(FairspecModel):
    identifier: str
    relation: str
    scheme: str


class ZenodoCommunity(FairspecModel):
    identifier: str


class ZenodoMetadata(FairspecModel):
    title: str
    description: str
    upload_type: str
    publication_date: str | None = None
    creators: list[ZenodoCreator]
    access_right: str | None = None
    license: str | None = None
    doi: str | None = None
    keywords: list[str] | None = None
    related_identifiers: list[ZenodoRelatedIdentifier] | None = None
    communities: list[ZenodoCommunity] | None = None
    version: str | None = None
