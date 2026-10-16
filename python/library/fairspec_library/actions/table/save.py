from __future__ import annotations

from typing import Unpack

from fairspec_table import TablePlugin
from fairspec_table.models.table import SaveTableOptions, Table

from fairspec_library.system import system


def save_table(table: Table, **options: Unpack[SaveTableOptions]) -> str | None:
    for plugin in system.plugins:
        if isinstance(plugin, TablePlugin):
            result = plugin.save_table(table, **options)
            if result is not None:
                return result

    return None
