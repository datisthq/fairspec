from __future__ import annotations

import urllib.parse
from collections.abc import Callable
from typing import TYPE_CHECKING

from fairspec_metadata import (
    denormalize_dataset,
    get_file_extension,
    stringify_descriptor,
)

from fairspec_dataset.actions.dataset.basepath import get_dataset_basepath
from fairspec_dataset.actions.resource.save import SaveFileProps, save_resource_files
from fairspec_dataset.actions.stream.load import load_file_stream

from fairspec_dataset.plugins.ckan.actions.resource.to_ckan import (
    convert_resource_to_ckan,
)
from fairspec_dataset.plugins.ckan.services.ckan import make_ckan_api_request
from .to_ckan import convert_dataset_to_ckan

if TYPE_CHECKING:
    from fairspec_metadata import Dataset
    from fairspec_metadata import Descriptor
    from fairspec_metadata import Resource


def save_dataset_to_ckan(
    dataset: Dataset,
    *,
    api_key: str,
    ckan_url: str,
    owner_org: str,
    dataset_name: str,
) -> dict:
    basepath = get_dataset_basepath(dataset)
    ckan_dataset = convert_dataset_to_ckan(dataset)

    payload = {
        **ckan_dataset.model_dump(by_alias=True, exclude_none=True),
        "name": dataset_name,
        "owner_org": owner_org,
        "resources": [],
    }

    result = make_ckan_api_request(
        action="package_create",
        payload=payload,
        ckan_url=ckan_url,
        api_key=api_key,
    )

    parsed = urllib.parse.urlparse(ckan_url)
    dataset_url = f"{parsed.scheme}://{parsed.netloc}/dataset/{result['name']}"

    resource_descriptors: list[Descriptor] = []
    for resource in dataset.resources or []:

        def _make_save_file(res: Resource) -> Callable[[SaveFileProps], str]:
            def _save_file(props: SaveFileProps) -> str:
                ckan_resource = convert_resource_to_ckan(
                    res.model_dump(by_alias=True, exclude_none=True)
                )
                extension = get_file_extension(props.normalized_path)

                upload_payload: dict = {
                    **ckan_resource.model_dump(by_alias=True, exclude_none=True),
                    "package_id": dataset_name,
                    "name": props.denormalized_path,
                }
                if extension:
                    upload_payload["format"] = extension.upper()

                stream = load_file_stream(props.normalized_path)
                file_data = stream.read()

                upload_result = make_ckan_api_request(
                    action="resource_create",
                    payload=upload_payload,
                    upload=(props.denormalized_path, file_data),
                    ckan_url=ckan_url,
                    api_key=api_key,
                )

                return upload_result["url"]

            return _save_file

        resource_descriptors.append(
            save_resource_files(
                resource,
                basepath=basepath,
                with_remote=True,
                without_folders=True,
                save_file=_make_save_file(resource),
            )
        )

    denormalized = denormalize_dataset(dataset, basepath=basepath)
    descriptor: Descriptor = {
        **denormalized.model_dump(by_alias=True, exclude_none=True),
        "resources": resource_descriptors,
    }

    descriptor_bytes = stringify_descriptor(descriptor).encode()

    make_ckan_api_request(
        action="resource_create",
        payload={
            "package_id": dataset_name,
            "name": "datapackage.json",
        },
        upload=("datapackage.json", descriptor_bytes),
        ckan_url=ckan_url,
        api_key=api_key,
    )

    return {
        "path": result.get("url"),
        "dataset_url": dataset_url,
    }
