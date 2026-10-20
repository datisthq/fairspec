from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class GithubFile(FairspecModel):
    path: str
    mode: str
    type: str
    size: int | None = None
    sha: str
    url: str
