from __future__ import annotations

import base64
from collections.abc import Callable
from typing import TYPE_CHECKING

from fairspec_metadata import denormalize_dataset, stringify_descriptor

from fairspec_dataset.actions.dataset.basepath import get_dataset_basepath
from fairspec_dataset.actions.resource.save import SaveFileProps, save_resource_files
from fairspec_dataset.actions.stream.load import load_file_stream

from fairspec_dataset.plugins.github.services.github import make_github_api_request

if TYPE_CHECKING:
    from fairspec_metadata import Dataset
    from fairspec_metadata import Descriptor
    from fairspec_metadata import Resource


def save_dataset_to_github(
    dataset: Dataset,
    *,
    api_key: str,
    repo: str,
    org: str | None = None,
) -> dict:
    basepath = get_dataset_basepath(dataset)

    endpoint = f"/orgs/{org}/repos" if org else "/user/repos"
    github_repository = make_github_api_request(
        endpoint=endpoint,
        payload={"name": repo, "auto_init": True},
        method="POST",
        api_key=api_key,
    )

    owner_login = github_repository["owner"]["login"]

    resource_descriptors: list[Descriptor] = []
    for resource in dataset.resources or []:

        def _make_save_file(res: Resource) -> Callable[[SaveFileProps], str]:
            def _save_file(props: SaveFileProps) -> str:
                stream = load_file_stream(props.normalized_path)
                content = base64.b64encode(stream.read()).decode()

                payload = {
                    "path": props.denormalized_path,
                    "content": content,
                    "message": f'Added file "{props.denormalized_path}"',
                }

                make_github_api_request(
                    endpoint=f"/repos/{owner_login}/{repo}/contents/{props.denormalized_path}",
                    method="PUT",
                    payload=payload,
                    api_key=api_key,
                )

                return props.denormalized_path

            return _save_file

        resource_descriptors.append(
            save_resource_files(
                resource,
                basepath=basepath,
                with_remote=False,
                save_file=_make_save_file(resource),
            )
        )

    denormalized = denormalize_dataset(dataset, basepath=basepath)
    descriptor: Descriptor = {
        **denormalized.model_dump(by_alias=True, exclude_none=True),
        "resources": resource_descriptors,
    }

    content = base64.b64encode(stringify_descriptor(descriptor).encode()).decode()

    make_github_api_request(
        endpoint=f"/repos/{owner_login}/{repo}/contents/dataset.json",
        method="PUT",
        payload={
            "path": "dataset.json",
            "message": 'Added file "dataset.json"',
            "content": content,
        },
        api_key=api_key,
    )

    return {
        "path": f"https://raw.githubusercontent.com/{owner_login}/{repo}/refs/heads/main/dataset.json",
        "repo_url": github_repository["html_url"],
    }
