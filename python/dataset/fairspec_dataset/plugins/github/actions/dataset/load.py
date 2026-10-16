from __future__ import annotations

import urllib.parse
from typing import TYPE_CHECKING

from fairspec_dataset.actions.dataset.merge import merge_datasets
from fairspec_dataset.plugins.github.models.repository import GithubRepository

from fairspec_dataset.plugins.github.services.github import make_github_api_request
from .from_github import convert_dataset_from_github

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def load_dataset_from_github(
    repo_url: str,
    *,
    api_key: str | None = None,
) -> Descriptor:
    owner, repo = _extract_repository_info(repo_url)
    if not owner or not repo:
        raise Exception(f"Failed to extract repository info from URL: {repo_url}")

    repository = make_github_api_request(
        endpoint=f"/repos/{owner}/{repo}",
        api_key=api_key,
    )

    ref = repository["default_branch"]
    tree_response = make_github_api_request(
        endpoint=f"/repos/{owner}/{repo}/git/trees/{ref}?recursive=1",
        api_key=api_key,
    )
    repository["files"] = tree_response["tree"]

    system_dataset = convert_dataset_from_github(GithubRepository(**repository))
    user_dataset_path: str | None = None
    for resource in system_dataset.resources or []:
        custom = resource.unstable_customMetadata or {}
        if custom.get("githubKey") == "dataset.json":
            user_dataset_path = custom.get("githubUrl")
            break

    dataset = merge_datasets(
        system_dataset=system_dataset,
        user_dataset_path=user_dataset_path,
    )

    for resource in dataset.resources or []:
        resource.unstable_customMetadata = None

    return dataset.model_dump(by_alias=True, exclude_none=True)


def _extract_repository_info(repo_url: str) -> tuple[str | None, str | None]:
    parsed = urllib.parse.urlparse(repo_url)
    parts = [p for p in parsed.path.split("/") if p]
    if len(parts) >= 2:
        return parts[0], parts[1]
    return None, None
