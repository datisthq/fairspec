from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .creator import ZenodoCreator
from .file import ZenodoFile


class ZenodoRecordLicense(FairspecModel):
    id: str


class ZenodoRecordMetadata(FairspecModel):
    title: str
    description: str | None = None
    creators: list[ZenodoCreator]
    publication_date: str | None = None
    license: ZenodoRecordLicense | None = None
    doi: str | None = None
    keywords: list[str] | None = None
    version: str | None = None


class ZenodoRecord(FairspecModel):
    id: int
    metadata: ZenodoRecordMetadata
    files: list[ZenodoFile] | None = None
