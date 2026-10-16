from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from fairspec_metadata import Integrity


class FileDescription(FairspecModel):
    bytes: int
    textual: bool
    integrity: Integrity | None
