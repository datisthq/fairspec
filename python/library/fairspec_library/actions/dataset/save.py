from __future__ import annotations

from typing import Unpack

from fairspec_dataset import DatasetPlugin, SaveDatasetOptions, SaveDatasetResult
from fairspec_metadata import Dataset

from fairspec_library.system import system


def save_dataset(
    dataset: Dataset, **options: Unpack[SaveDatasetOptions]
) -> SaveDatasetResult | None:
    for plugin in system.plugins:
        if isinstance(plugin, DatasetPlugin):
            result = plugin.save_dataset(dataset, **options)
            if result is not None:
                return result

    return None
