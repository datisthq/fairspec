from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .file import GithubFile


class GithubTree(FairspecModel):
    tree: list[GithubFile]
