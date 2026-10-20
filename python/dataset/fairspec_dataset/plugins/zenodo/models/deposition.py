from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .link import ZenodoLink


class ZenodoDeposition(FairspecModel):
    id: int
    links: ZenodoLink
    state: str | None = None
    submitted: bool | None = None
