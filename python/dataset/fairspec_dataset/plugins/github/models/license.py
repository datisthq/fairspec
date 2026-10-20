from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class GithubLicense(FairspecModel):
    key: str
    name: str
    spdx_id: str | None
    url: str | None
