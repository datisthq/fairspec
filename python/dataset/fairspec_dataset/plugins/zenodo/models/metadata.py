from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .creator import ZenodoCreator


class ZenodoMetadata(FairspecModel):
    title: str | None = None
    description: str | None = None
    upload_type: str | None = None
    publication_date: str | None = None
    creators: list[ZenodoCreator] | None = None
    access_right: str | None = None
    license: str | None = None
    doi: str | None = None
    keywords: list[str] | None = None
    related_identifiers: list[dict] | None = None
    communities: list[dict] | None = None
    version: str | None = None
