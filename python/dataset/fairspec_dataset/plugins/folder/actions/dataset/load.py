from __future__ import annotations

import os

from fairspec_metadata import load_dataset_descriptor
from fairspec_metadata import Dataset


def load_dataset_from_folder(folder_path: str) -> Dataset:
    return load_dataset_descriptor(os.path.join(folder_path, "dataset.json"))
