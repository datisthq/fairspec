from __future__ import annotations

import urllib.parse
from typing import TYPE_CHECKING

from fairspec_metadata import denormalize_dataset, stringify_descriptor

from fairspec_dataset.actions.dataset.basepath import get_dataset_basepath
from fairspec_dataset.actions.resource.save import (
    SaveFileCallback,
    SaveFileProps,
    save_resource_files,
)
from fairspec_dataset.actions.stream.load import load_file_stream

from fairspec_dataset.plugins.zenodo.services.zenodo import make_zenodo_api_request
from .to_zenodo import convert_dataset_to_zenodo

if TYPE_CHECKING:
    from fairspec_metadata import Dataset
    from fairspec_metadata import Descriptor
    from fairspec_metadata import Resource


def save_dataset_to_zenodo(
    dataset: Dataset,
    *,
    api_key: str,
    sandbox: bool = False,
) -> dict:
    basepath = get_dataset_basepath(dataset)

    new_zenodo_record = convert_dataset_to_zenodo(dataset)
    zenodo_record = make_zenodo_api_request(
        payload=new_zenodo_record,
        endpoint="/deposit/depositions",
        method="POST",
        api_key=api_key,
        sandbox=sandbox,
    )

    record_id = zenodo_record["id"]

    resource_descriptors: list[Descriptor] = []
    for resource in dataset.resources or []:

        def _make_save_file(res: Resource) -> SaveFileCallback:
            def _save_file(props: SaveFileProps) -> str:
                stream = load_file_stream(props.normalized_path)
                file_data = stream.read()

                make_zenodo_api_request(
                    endpoint=f"/deposit/depositions/{record_id}/files",
                    method="POST",
                    upload=(props.denormalized_path, file_data),
                    api_key=api_key,
                    sandbox=sandbox,
                )

                return props.denormalized_path

            return _save_file

        resource_descriptors.append(
            save_resource_files(
                resource,
                basepath=basepath,
                with_remote=False,
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

    make_zenodo_api_request(
        endpoint=f"/deposit/depositions/{record_id}/files",
        method="POST",
        upload=("dataset.json", descriptor_bytes),
        api_key=api_key,
        sandbox=sandbox,
    )

    html_url = zenodo_record["links"]["html"]
    parsed = urllib.parse.urlparse(html_url)
    origin = f"{parsed.scheme}://{parsed.netloc}"

    return {
        "path": f"{origin}/records/{record_id}/files/dataset.json",
        "dataset_url": f"{origin}/uploads/{record_id}",
    }
