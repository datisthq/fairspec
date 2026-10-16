from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_metadata import get_is_remote_path

from fairspec_dataset.plugin import DatasetPlugin
from .actions.dataset.load import load_dataset_from_ckan

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


class CkanPlugin(DatasetPlugin):
    def load_dataset(self, source: str) -> Descriptor | None:
        if not _get_is_ckan(source):
            return None
        return load_dataset_from_ckan(source)


def _get_is_ckan(path: str) -> bool:
    if not get_is_remote_path(path):
        return False
    return "/dataset/" in path
