from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class ZenodoLink(FairspecModel):
    self: str | None = None
    html: str | None = None
    files: str | None = None
    bucket: str | None = None
    publish: str | None = None
    discard: str | None = None
    edit: str | None = None
