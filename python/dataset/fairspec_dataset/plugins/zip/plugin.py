from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import get_file_extension

from fairspec_dataset.models.dataset import SaveDatasetResult
from fairspec_dataset.plugin import DatasetPlugin
from .actions.dataset.load import load_dataset_from_zip
from .actions.dataset.save import save_dataset_to_zip

if TYPE_CHECKING:
    from fairspec_metadata import Dataset
    from fairspec_metadata import Descriptor

    from fairspec_dataset.models.dataset import SaveDatasetOptions


class ZipPlugin(DatasetPlugin):
    def load_dataset(self, source: str) -> Descriptor | None:
        if not _get_is_zip(source):
            return None
        dataset = load_dataset_from_zip(source)
        return dataset.model_dump(by_alias=True, exclude_none=True)

    def save_dataset(
        self, dataset: Dataset, **options: Unpack[SaveDatasetOptions]
    ) -> SaveDatasetResult | None:
        target = options["target"]
        if not _get_is_zip(target):
            return None
        save_dataset_to_zip(
            dataset, archive_path=target, with_remote=bool(options.get("with_remote"))
        )
        return SaveDatasetResult(path=None)


def _get_is_zip(path: str) -> bool:
    return get_file_extension(path) == "zip"
