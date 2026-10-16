from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata.actions.descriptor.save import save_descriptor
from fairspec_metadata.actions.path.basepath import get_basepath
from fairspec_metadata.settings import FAIRSPEC_VERSION

from .denormalize import denormalize_dataset

if TYPE_CHECKING:
    from fairspec_metadata.models.dataset import Dataset


def save_dataset_descriptor(
    dataset: Dataset,
    *,
    path: str,
    overwrite: bool = False,
) -> None:
    basepath = get_basepath(path)
    denormalized = denormalize_dataset(dataset, basepath=basepath)
    descriptor = denormalized.model_dump(by_alias=True, exclude_none=True)

    if "$schema" not in descriptor:
        descriptor["$schema"] = (
            f"https://fairspec.org/profiles/{FAIRSPEC_VERSION}/dataset.json"
        )

    save_descriptor(descriptor, path=path, overwrite=overwrite)
