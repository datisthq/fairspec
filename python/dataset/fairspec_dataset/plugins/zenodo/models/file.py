from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class ZenodoFileLinks(FairspecModel):
    self: str


class ZenodoFile(FairspecModel):
    id: str
    key: str
    size: int
    checksum: str
    links: ZenodoFileLinks
