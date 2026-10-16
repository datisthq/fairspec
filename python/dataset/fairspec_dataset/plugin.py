from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata.plugin import MetadataPlugin

if TYPE_CHECKING:
    from fairspec_metadata import Dataset
    from fairspec_metadata import Descriptor
    from fairspec_metadata import FileDialect
    from fairspec_metadata import Resource

    from .models.dataset import SaveDatasetOptions, SaveDatasetResult
    from .models.file_dialect import InferFileDialectOptions


class DatasetPlugin(MetadataPlugin):
    def load_dataset(self, source: str) -> Descriptor | None:
        return None

    def save_dataset(
        self, dataset: Dataset, **options: Unpack[SaveDatasetOptions]
    ) -> SaveDatasetResult | None:
        return None

    def infer_file_dialect(
        self,
        resource: Resource,
        **options: Unpack[InferFileDialectOptions],
    ) -> FileDialect | None:
        return None
