from __future__ import annotations

import urllib.parse
from typing import TYPE_CHECKING

from fairspec_dataset.actions.dataset.merge import merge_datasets

from fairspec_dataset.plugins.ckan.models.dataset import CkanDataset
from fairspec_dataset.plugins.ckan.services.ckan import make_ckan_api_request
from .from_ckan import convert_dataset_from_ckan

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def load_dataset_from_ckan(
    dataset_url: str,
    *,
    api_key: str | None = None,
) -> Descriptor:
    dataset_id = _extract_dataset_id(dataset_url)
    if not dataset_id:
        raise Exception(f"Failed to extract dataset ID from URL: {dataset_url}")

    ckan_dict = make_ckan_api_request(
        ckan_url=dataset_url,
        action="package_show",
        payload={"id": dataset_id},
        api_key=api_key,
    )

    for resource in ckan_dict.get("resources", []):
        resource_id = resource.get("id")
        if resource.get("format", "").upper() in ("CSV", "XLS", "XLSX"):
            schema = _load_ckan_schema(
                dataset_url=dataset_url,
                resource_id=resource_id,
                api_key=api_key,
            )
            if schema:
                resource["schema"] = schema

    ckan_dataset = CkanDataset(**ckan_dict)
    system_dataset = convert_dataset_from_ckan(ckan_dataset)
    user_dataset_path: str | None = None
    for resource in system_dataset.resources or []:
        custom = resource.unstable_customMetadata or {}
        if custom.get("ckanKey") == "dataset.json":
            user_dataset_path = custom.get("ckanUrl")
            break

    dataset = merge_datasets(
        system_dataset=system_dataset,
        user_dataset_path=user_dataset_path,
    )

    for resource in dataset.resources or []:
        resource.unstable_customMetadata = None

    return dataset.model_dump(by_alias=True, exclude_none=True)


def _extract_dataset_id(dataset_url: str) -> str | None:
    parsed = urllib.parse.urlparse(dataset_url)
    parts = [p for p in parsed.path.split("/") if p]
    return parts[-1] if parts else None


def _load_ckan_schema(
    *,
    dataset_url: str,
    resource_id: str,
    api_key: str | None = None,
) -> dict | None:
    try:
        result = make_ckan_api_request(
            ckan_url=dataset_url,
            action="datastore_search",
            payload={"resource_id": resource_id, "limit": 0},
            api_key=api_key,
        )
        fields = [
            f
            for f in result.get("fields", [])
            if f.get("id") not in ("_id", "_full_text")
        ]
        return {"fields": fields}
    except Exception:
        return None
