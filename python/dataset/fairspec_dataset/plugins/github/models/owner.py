from __future__ import annotations

from typing import Literal

from fairspec_metadata.models.base import FairspecModel


class GithubOwner(FairspecModel):
    login: str
    id: int
    avatar_url: str
    html_url: str
    type: Literal["User", "Organization"]
