from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class CkanTag(FairspecModel):
    id: str | None = None
    name: str
    display_name: str | None = None
