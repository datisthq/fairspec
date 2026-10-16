from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class CkanFieldInfo(FairspecModel):
    label: str | None = None
    notes: str | None = None
    type_override: str | None = None


class CkanField(FairspecModel):
    id: str | None = None
    type: str | None = None
    info: CkanFieldInfo | None = None
