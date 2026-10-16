from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import load_dataset_descriptor
from fairspec_metadata import save_dataset_descriptor
from fairspec_metadata import get_file_extension, get_is_remote_path

from fairspec_dataset.models.dataset import SaveDatasetResult
from fairspec_dataset.plugin import DatasetPlugin

if TYPE_CHECKING:
    from fairspec_metadata import Dataset
    from fairspec_metadata import Descriptor

    from fairspec_dataset.models.dataset import SaveDatasetOptions


class DescriptorPlugin(DatasetPlugin):
    def load_dataset(self, source: str) -> Descriptor | None:
        if not _get_is_json(source):
            return None
        dataset = load_dataset_descriptor(source)
        return dataset.model_dump(by_alias=True, exclude_none=True)

    def save_dataset(
        self, dataset: Dataset, **options: Unpack[SaveDatasetOptions]
    ) -> SaveDatasetResult | None:
        target = options["target"]
        if not _get_is_local_json(target):
            return None
        if not target.endswith("datapackage.json"):
            return None
        save_dataset_descriptor(dataset, path=target)
        return SaveDatasetResult(path=target)


def _get_is_json(path: str) -> bool:
    return get_file_extension(path) == "json"


def _get_is_local_json(path: str) -> bool:
    return _get_is_json(path) and not get_is_remote_path(path)
