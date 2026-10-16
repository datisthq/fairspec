from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class GithubOwner(FairspecModel):
    login: str | None = None
    id: int | None = None
    avatar_url: str | None = None
    html_url: str | None = None
    type: str | None = None
