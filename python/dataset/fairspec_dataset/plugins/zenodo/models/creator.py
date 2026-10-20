from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class ZenodoCreatorIdentifier(FairspecModel):
    identifier: str
    scheme: str


class ZenodoCreator(FairspecModel):
    name: str
    affiliation: str | None = None
    identifiers: list[ZenodoCreatorIdentifier] | None = None
