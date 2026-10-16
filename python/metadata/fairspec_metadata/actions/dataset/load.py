from __future__ import annotations

from fairspec_metadata.actions.descriptor.load import load_descriptor
from fairspec_metadata.actions.path.basepath import resolve_basepath
from fairspec_metadata.models.dataset import Dataset

from .assert_ import assert_dataset


def load_dataset_descriptor(path: str) -> Dataset:
    basepath = resolve_basepath(path)
    descriptor = load_descriptor(path)
    return assert_dataset(descriptor, basepath=basepath)
