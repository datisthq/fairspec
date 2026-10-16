from __future__ import annotations

import urllib.parse
from typing import TYPE_CHECKING

from fairspec_dataset.actions.dataset.merge import merge_datasets
from fairspec_dataset.plugins.zenodo.models.record import ZenodoRecord

from fairspec_dataset.plugins.zenodo.services.zenodo import make_zenodo_api_request
from .from_zenodo import convert_dataset_from_zenodo

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def load_dataset_from_zenodo(
    dataset_url: str,
    *,
    api_key: str | None = None,
) -> Descriptor:
    parsed = urllib.parse.urlparse(dataset_url)
    sandbox = parsed.hostname == "sandbox.zenodo.org"

    record_id = _extract_record_id(dataset_url)
    if not record_id:
        raise Exception(f"Failed to extract record ID from URL: {dataset_url}")

    zenodo_record = make_zenodo_api_request(
        endpoint=f"/records/{record_id}",
        api_key=api_key,
        sandbox=sandbox,
    )

    system_dataset = convert_dataset_from_zenodo(ZenodoRecord(**zenodo_record))
    user_dataset_path: str | None = None
    for resource in system_dataset.resources or []:
        custom = resource.unstable_customMetadata or {}
        if custom.get("zenodoKey") == "dataset.json":
            user_dataset_path = custom.get("zenodoUrl")
            break

    dataset = merge_datasets(
        system_dataset=system_dataset,
        user_dataset_path=user_dataset_path,
    )

    for resource in dataset.resources or []:
        resource.unstable_customMetadata = None

    return dataset.model_dump(by_alias=True, exclude_none=True)


def _extract_record_id(dataset_url: str) -> str | None:
    parsed = urllib.parse.urlparse(dataset_url)
    parts = [p for p in parsed.path.split("/") if p]
    return parts[-1] if parts else None
