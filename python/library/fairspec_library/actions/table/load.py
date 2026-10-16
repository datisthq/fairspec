from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_table import TablePlugin
from fairspec_table.models.table import LoadTableOptions

from fairspec_library.system import system

if TYPE_CHECKING:
    from fairspec_metadata import Resource
    from fairspec_table import Table


def load_table(resource: Resource, **options: Unpack[LoadTableOptions]) -> Table | None:
    for plugin in system.plugins:
        if isinstance(plugin, TablePlugin):
            result = plugin.load_table(resource, **options)
            if result is not None:
                return result

    return None
