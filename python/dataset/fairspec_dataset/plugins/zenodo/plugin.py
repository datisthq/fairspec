from __future__ import annotations

import urllib.parse
from typing import TYPE_CHECKING

from fairspec_metadata import get_is_remote_path

from fairspec_dataset.plugin import DatasetPlugin
from .actions.dataset.load import load_dataset_from_zenodo

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


class ZenodoPlugin(DatasetPlugin):
    def load_dataset(self, source: str) -> Descriptor | None:
        if not _get_is_zenodo(source):
            return None
        return load_dataset_from_zenodo(source)


def _get_is_zenodo(path: str) -> bool:
    if not get_is_remote_path(path):
        return False
    hostname = urllib.parse.urlparse(path).hostname or ""
    return hostname.endswith("zenodo.org")
