from __future__ import annotations

from enum import StrEnum

from .base import FairspecModel


class IntegrityType(StrEnum):
    md5 = "md5"
    sha1 = "sha1"
    sha256 = "sha256"
    sha512 = "sha512"


class Integrity(FairspecModel):
    type: IntegrityType
    hash: str
