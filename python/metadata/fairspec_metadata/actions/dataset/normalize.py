from __future__ import annotations

from fairspec_metadata.actions.resource.normalize import normalize_resource
from fairspec_metadata.models.dataset import Dataset


def normalize_dataset(dataset: Dataset, *, basepath: str | None = None) -> Dataset:
    result = dataset.model_dump(by_alias=True, exclude_none=True)

    if dataset.resources:
        result["resources"] = [
            normalize_resource(resource, basepath=basepath)
            for resource in dataset.resources
        ]

    return Dataset(**result)
