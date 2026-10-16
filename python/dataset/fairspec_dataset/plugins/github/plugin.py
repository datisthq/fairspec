from __future__ import annotations

import urllib.parse
from typing import TYPE_CHECKING

from fairspec_metadata import get_is_remote_path

from fairspec_dataset.plugin import DatasetPlugin
from .actions.dataset.load import load_dataset_from_github

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


class GithubPlugin(DatasetPlugin):
    def load_dataset(self, source: str) -> Descriptor | None:
        if not _get_is_github(source):
            return None
        return load_dataset_from_github(source)


def _get_is_github(path: str) -> bool:
    if not get_is_remote_path(path):
        return False
    return urllib.parse.urlparse(path).hostname == "github.com"
