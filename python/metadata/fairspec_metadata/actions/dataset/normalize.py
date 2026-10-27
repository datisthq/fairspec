from __future__ import annotations

from fairspec_metadata.actions.resource.normalize import normalize_resource
from fairspec_metadata.models.dataset import Dataset

PATH_PROPERTIES = ("data", "fileDialect", "dataSchema", "tableSchema")


def normalize_dataset(dataset: Dataset, *, basepath: str | None = None) -> Dataset:
    if not dataset.resources:
        return dataset.model_copy(deep=True)

    # Copy instead of rebuilding: a rebuild re-checks the spec's Path pattern, which a
    # path made relative to the working directory (`../data/schema.json`) fails
    resources = []
    for resource in dataset.resources:
        normalized = normalize_resource(resource, basepath=basepath)
        update = {
            name: normalized[name]
            for name in PATH_PROPERTIES
            if isinstance(getattr(resource, name, None), (str, list))
        }
        resources.append(resource.model_copy(update=update, deep=True))

    return dataset.model_copy(update={"resources": resources}, deep=True)
