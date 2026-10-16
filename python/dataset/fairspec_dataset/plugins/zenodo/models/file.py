from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class ZenodoFileLinks(FairspecModel):
    self: str | None = None


class ZenodoFile(FairspecModel):
    id: str | None = None
    key: str | None = None
    size: int | None = None
    checksum: str | None = None
    links: ZenodoFileLinks | None = None
