from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_metadata import get_data_records

from fairspec_table.plugin import TablePlugin

from .actions.table.load import load_inline_table

if TYPE_CHECKING:
    from fairspec_metadata import Resource

    from fairspec_table.models.table import LoadTableOptions, Table


class InlinePlugin(TablePlugin):
    def load_table(
        self,
        resource: Resource,
        **options: Unpack[LoadTableOptions],
    ) -> Table | None:
        records = get_data_records(resource)
        if not records:
            return None
        return load_inline_table(resource, **options)
