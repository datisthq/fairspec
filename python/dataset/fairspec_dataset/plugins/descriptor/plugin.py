from __future__ import annotations

from collections.abc import Callable
from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import (
    convert_descriptor_from_frictionless,
    create_report,
    get_file_extension,
    get_is_frictionless_descriptor,
    get_is_remote_path,
    load_dataset_descriptor,
    load_descriptor,
    map_frictionless_errors,
    resolve_basepath,
    save_dataset_descriptor,
    validate_dataset_descriptor,
)

from fairspec_dataset.models.dataset import SaveDatasetResult
from fairspec_dataset.plugin import DatasetPlugin

if TYPE_CHECKING:
    from fairspec_metadata import Dataset
    from fairspec_metadata import Descriptor
    from fairspec_metadata import Report

    from fairspec_dataset.models.dataset import SaveDatasetOptions


class DescriptorPlugin(DatasetPlugin):
    def load_dataset(self, source: str) -> Descriptor | None:
        if not _get_is_json(source):
            return None
        dataset = load_dataset_descriptor(source)
        return dataset.model_dump(by_alias=True, exclude_none=True)

    def validate_dataset(
        self, source: str, *, validate_dataset: Callable[[Dataset], Report]
    ) -> Report | None:
        if not _get_is_json(source):
            return None

        basepath = resolve_basepath(source)
        descriptor = load_descriptor(source)

        report = validate_dataset_descriptor(descriptor, basepath=basepath)
        if report.dataset is None:
            return create_report(report.errors)

        dataset_report = validate_dataset(report.dataset)
        if not get_is_frictionless_descriptor(descriptor, kind="dataset"):
            return dataset_report

        mapping = convert_descriptor_from_frictionless(descriptor, kind="dataset").mapping
        return create_report(map_frictionless_errors(dataset_report.errors, mapping))

    def save_dataset(
        self, dataset: Dataset, **options: Unpack[SaveDatasetOptions]
    ) -> SaveDatasetResult | None:
        target = options["target"]
        if not _get_is_local_json(target):
            return None
        save_dataset_descriptor(dataset, path=target)
        return SaveDatasetResult(path=target)


def _get_is_json(path: str) -> bool:
    return get_file_extension(path) == "json"


def _get_is_local_json(path: str) -> bool:
    return _get_is_json(path) and not get_is_remote_path(path)
