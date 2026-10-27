from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import get_file_name, get_is_remote_path

from fairspec_dataset.plugin import DatasetPlugin
from .actions.dataset.save import save_dataset_to_frictionless

if TYPE_CHECKING:
    from fairspec_metadata import Dataset

    from fairspec_dataset.models.dataset import SaveDatasetOptions, SaveDatasetResult


class FrictionlessPlugin(DatasetPlugin):
    def save_dataset(
        self, dataset: Dataset, **options: Unpack[SaveDatasetOptions]
    ) -> SaveDatasetResult | None:
        target = options["target"]
        if not _get_is_datapackage(target) or get_is_remote_path(target):
            return None
        return save_dataset_to_frictionless(dataset, target=target)


def _get_is_datapackage(path: str) -> bool:
    return get_file_name(path) == "datapackage.json"
