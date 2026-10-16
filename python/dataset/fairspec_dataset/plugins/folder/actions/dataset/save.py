from __future__ import annotations

import os
from typing import TYPE_CHECKING

from fairspec_metadata import denormalize_dataset
from fairspec_metadata import save_descriptor

from fairspec_dataset.actions.dataset.basepath import get_dataset_basepath
from fairspec_dataset.actions.file.copy import copy_file
from fairspec_dataset.actions.file.path import assert_local_path_vacant
from fairspec_dataset.actions.folder.create import create_folder
from fairspec_dataset.actions.resource.save import SaveFileProps, save_resource_files

if TYPE_CHECKING:
    from fairspec_metadata import Dataset
    from fairspec_metadata import Descriptor


def save_dataset_to_folder(
    dataset: Dataset,
    *,
    folder_path: str,
    with_remote: bool = False,
) -> Descriptor:
    basepath = get_dataset_basepath(dataset)

    assert_local_path_vacant(folder_path)
    create_folder(folder_path)

    resource_descriptors: list[Descriptor] = []
    for resource in dataset.resources or []:
        resource_descriptors.append(
            save_resource_files(
                resource,
                basepath=basepath,
                with_remote=with_remote,
                save_file=_make_save_file(folder_path),
            )
        )

    denormalized = denormalize_dataset(dataset, basepath=basepath)
    descriptor: Descriptor = {
        **denormalized.model_dump(by_alias=True, exclude_none=True),
        "resources": resource_descriptors,
    }

    save_descriptor(descriptor, path=os.path.join(folder_path, "dataset.json"))

    return descriptor


def _make_save_file(folder_path: str):
    def save_file(props: SaveFileProps) -> str:
        copy_file(
            source_path=props.normalized_path,
            target_path=os.path.join(folder_path, props.denormalized_path),
        )
        return props.denormalized_path

    return save_file
