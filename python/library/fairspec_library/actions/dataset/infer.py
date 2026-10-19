from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import Dataset

from fairspec_dataset.helpers.concurrency import run_concurrent_tasks
from fairspec_table.models.schema import InferTableSchemaOptions

from fairspec_library.actions.resource.infer import infer_resource

if TYPE_CHECKING:
    from fairspec_metadata import Resource


def infer_dataset(
    dataset: Dataset, **options: Unpack[InferTableSchemaOptions]
) -> Dataset:
    dataset = dataset.model_copy(deep=True)

    if dataset.resources:

        def infer(entry: tuple[int, Resource]) -> Resource:
            index, resource = entry
            return infer_resource(resource, resource_number=index + 1, **options)

        dataset.resources[:] = run_concurrent_tasks(
            infer,
            list(enumerate(dataset.resources)),
            concurrency=options.get("concurrency"),
        )

    return dataset
