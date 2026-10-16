from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class CkanTag(FairspecModel):
    id: str | None = None
    name: str | None = None
    display_name: str | None = None
