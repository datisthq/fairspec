from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import (
    convert_dataset_to_frictionless,
    denormalize_resource,
    get_basepath,
    resolve_file_dialect,
    resolve_table_schema,
    save_descriptor,
)

from fairspec_dataset.models.dataset import SaveDatasetResult

if TYPE_CHECKING:
    from fairspec_metadata import Dataset


def save_dataset_to_frictionless(
    dataset: Dataset, *, target: str, overwrite: bool = False
) -> SaveDatasetResult:
    """Save a Dataset as a Frictionless Data Package (v2) descriptor

    Paths are written relative to the descriptor; data files are not copied
    """
    basepath = get_basepath(target)

    resources = []
    for resource in dataset.resources or []:
        table_schema = resolve_table_schema(resource.tableSchema)
        file_dialect = resolve_file_dialect(resource.fileDialect)
        resources.append(
            denormalize_resource(
                resource.model_copy(
                    update={"tableSchema": table_schema, "fileDialect": file_dialect}
                ),
                basepath=basepath,
            )
        )

    descriptor = convert_dataset_to_frictionless(
        {
            **dataset.model_dump(mode="json", by_alias=True, exclude_none=True),
            "resources": resources,
        }
    )
    save_descriptor(descriptor, path=target, overwrite=overwrite)

    return SaveDatasetResult(path=target)
