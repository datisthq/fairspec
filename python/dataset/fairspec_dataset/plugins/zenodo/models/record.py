from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .file import ZenodoFile
from .link import ZenodoLink
from .metadata import ZenodoMetadata


class ZenodoRecord(FairspecModel):
    id: int | None = None
    links: ZenodoLink | None = None
    metadata: ZenodoMetadata | None = None
    files: list[ZenodoFile] | None = None
    state: str | None = None
    submitted: bool | None = None
