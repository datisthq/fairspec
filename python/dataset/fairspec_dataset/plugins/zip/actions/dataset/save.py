from __future__ import annotations

import zipfile
from typing import TYPE_CHECKING

from fairspec_metadata import denormalize_dataset
from fairspec_metadata import stringify_descriptor

from fairspec_dataset.actions.dataset.basepath import get_dataset_basepath
from fairspec_dataset.actions.file.path import assert_local_path_vacant
from fairspec_dataset.actions.resource.save import SaveFileProps, save_resource_files
from fairspec_dataset.actions.stream.load import load_file_stream

if TYPE_CHECKING:
    from fairspec_metadata import Dataset
    from fairspec_metadata import Descriptor


def save_dataset_to_zip(
    dataset: Dataset,
    *,
    archive_path: str,
    with_remote: bool = False,
) -> None:
    basepath = get_dataset_basepath(dataset)

    assert_local_path_vacant(archive_path)
    files: dict[str, bytes] = {}

    resource_descriptors: list[Descriptor] = []
    for resource in dataset.resources or []:
        resource_descriptors.append(
            save_resource_files(
                resource,
                basepath=basepath,
                with_remote=with_remote,
                save_file=_make_save_file(files),
            )
        )

    denormalized = denormalize_dataset(dataset, basepath=basepath)
    descriptor: Descriptor = {
        **denormalized.model_dump(by_alias=True, exclude_none=True),
        "resources": resource_descriptors,
    }

    files["dataset.json"] = stringify_descriptor(descriptor).encode("utf-8")

    with zipfile.ZipFile(archive_path, "w", zipfile.ZIP_DEFLATED) as zf:
        for name, data in files.items():
            zf.writestr(name, data)


def _make_save_file(files: dict[str, bytes]):
    def save_file(props: SaveFileProps) -> str:
        stream = load_file_stream(props.normalized_path)
        files[props.denormalized_path] = stream.read()
        return props.denormalized_path

    return save_file
