from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class ZenodoDepositionFile(FairspecModel):
    id: str
    filename: str | None = None
    filesize: int | None = None
    checksum: str | None = None
