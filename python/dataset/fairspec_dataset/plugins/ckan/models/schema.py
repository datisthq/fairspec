from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .field import CkanField


class CkanSchema(FairspecModel):
    fields: list[CkanField] | None = None
