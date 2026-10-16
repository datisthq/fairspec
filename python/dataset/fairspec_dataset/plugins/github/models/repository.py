from __future__ import annotations

from fairspec_metadata.models.base import FairspecModel

from .file import GithubFile
from .license import GithubLicense
from .owner import GithubOwner


class GithubRepository(FairspecModel):
    id: int | None = None
    name: str | None = None
    full_name: str | None = None
    owner: GithubOwner | None = None
    description: str | None = None
    created_at: str | None = None
    updated_at: str | None = None
    homepage: str | None = None
    size: int | None = None
    stargazers_count: int | None = None
    watchers_count: int | None = None
    language: str | None = None
    license: GithubLicense | None = None
    default_branch: str | None = None
    topics: list[str] | None = None
    private: bool | None = None
    archived: bool | None = None
    html_url: str | None = None
    git_url: str | None = None
    ssh_url: str | None = None
    clone_url: str | None = None
    files: list[GithubFile] | None = None
