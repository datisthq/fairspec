from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_dataset import DatasetPlugin
from fairspec_metadata import Dataset

from fairspec_library.system import system

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor

SCHEMA_PATH_PROPERTIES = ("fileDialect", "dataSchema", "tableSchema")


def load_dataset(source: str) -> Descriptor | None:
    for plugin in system.plugins:
        if isinstance(plugin, DatasetPlugin):
            result = plugin.load_dataset(source)
            if result is not None:
                return result

    return None


def load_dataset_model(source: str) -> Dataset:
    descriptor = load_dataset(source)
    if descriptor is None:
        raise ValueError(f"No plugin can load the dataset: {source}")

    # A loaded path is relative to the working directory (`../data/schema.json`) and
    # fails the spec's Path pattern, so it skips validation and is put back after
    paths: list[Descriptor] = []
    resources: list[Descriptor] = []
    for resource in descriptor.get("resources") or []:
        path = {
            name: value
            for name, value in resource.items()
            if name in SCHEMA_PATH_PROPERTIES and isinstance(value, str)
        }
        paths.append(path)
        resources.append({k: v for k, v in resource.items() if k not in path})

    if "resources" in descriptor:
        descriptor = {**descriptor, "resources": resources}
    dataset = Dataset.model_validate(descriptor)
    if dataset.resources is None:
        return dataset

    return dataset.model_copy(
        update={
            "resources": [
                resource.model_copy(update=path)
                for resource, path in zip(dataset.resources, paths)
            ]
        }
    )
