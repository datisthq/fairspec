from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class ZenodoCreator(FairspecModel):
    name: str | None = None
    affiliation: str | None = None
    identifiers: list[dict] | None = None
