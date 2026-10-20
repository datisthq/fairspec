from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel


class GithubContentFile(FairspecModel):
    path: str
    sha: str


class GithubContent(FairspecModel):
    content: GithubContentFile
