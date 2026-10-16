from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class GithubLicense(FairspecModel):
    key: str | None = None
    name: str | None = None
    spdx_id: str | None = None
    url: str | None = None
