from __future__ import annotations

import os
import zipfile

from fairspec_metadata import load_dataset_descriptor
from fairspec_metadata import Dataset

from fairspec_dataset.actions.folder.temp import get_temp_folder_path


def load_dataset_from_zip(archive_path: str) -> Dataset:
    basepath = get_temp_folder_path()

    with zipfile.ZipFile(archive_path, "r") as zf:
        zf.extractall(basepath)

    return load_dataset_descriptor(os.path.join(basepath, "dataset.json"))
