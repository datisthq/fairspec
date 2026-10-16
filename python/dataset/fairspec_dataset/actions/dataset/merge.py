from __future__ import annotations

from fairspec_metadata import load_dataset_descriptor
from fairspec_metadata import Dataset


def merge_datasets(
    *,
    system_dataset: Dataset,
    user_dataset_path: str | None = None,
) -> Dataset:
    system = system_dataset.model_dump(by_alias=True, exclude_none=True)

    user_dataset = (
        load_dataset_descriptor(user_dataset_path).model_dump(
            by_alias=True, exclude_none=True
        )
        if user_dataset_path
        else None
    )

    merged = {**system, **user_dataset} if user_dataset else {**system}
    return Dataset(**merged)
