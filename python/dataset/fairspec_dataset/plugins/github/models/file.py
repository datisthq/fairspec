from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class GithubFile(FairspecModel):
    path: str | None = None
    mode: str | None = None
    type: str | None = None
    size: int | None = None
    sha: str | None = None
    url: str | None = None
