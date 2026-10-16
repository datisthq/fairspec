from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class CkanOrganization(FairspecModel):
    id: str | None = None
    name: str | None = None
    title: str | None = None
    description: str | None = None
