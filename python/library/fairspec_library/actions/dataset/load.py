from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_dataset import DatasetPlugin

from fairspec_library.system import system

if TYPE_CHECKING:
    from fairspec_metadata import Descriptor


def load_dataset(source: str) -> Descriptor | None:
    for plugin in system.plugins:
        if isinstance(plugin, DatasetPlugin):
            result = plugin.load_dataset(source)
            if result is not None:
                return result

    return None
