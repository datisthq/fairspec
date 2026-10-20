from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .organization import CkanOrganization
from .resource import CkanResource, NewCkanResource
from .tag import CkanTag


class CkanDataset(FairspecModel):
    resources: list[CkanResource]
    organization: CkanOrganization | None = None
    tags: list[CkanTag]
    id: str
    name: str
    title: str | None = None
    notes: str | None = None
    version: str | None = None
    license_id: str | None = None
    license_title: str | None = None
    license_url: str | None = None
    author: str | None = None
    author_email: str | None = None
    maintainer: str | None = None
    maintainer_email: str | None = None
    metadata_created: str | None = None
    metadata_modified: str | None = None


class NewCkanDataset(FairspecModel):
    resources: list[NewCkanResource]
    tags: list[CkanTag]
    title: str | None = None
    notes: str | None = None
    version: str | None = None
    license_id: str | None = None
    license_title: str | None = None
    license_url: str | None = None
    author: str | None = None
    maintainer: str | None = None
    metadata_created: str | None = None
    metadata_modified: str | None = None
