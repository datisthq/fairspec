from __future__ import annotations

from enum import StrEnum

from fairspec_metadata.models.base import FairspecModel


class SampleSizeType(StrEnum):
    RECORDS = "Records"
    BYTES = "Bytes"
    ALL = "All"


class SampleSize(FairspecModel):
    type: SampleSizeType
    count: int = 0
