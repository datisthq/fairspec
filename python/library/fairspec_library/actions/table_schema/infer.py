from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_table import TablePlugin, infer_table_schema_from_table
from fairspec_table.models.schema import InferTableSchemaOptions

from fairspec_library.actions.table.load import load_table
from fairspec_library.system import system

if TYPE_CHECKING:
    from fairspec_metadata import Resource, TableSchema


def infer_table_schema(
    resource: Resource, **options: Unpack[InferTableSchemaOptions]
) -> TableSchema | None:
    for plugin in system.plugins:
        if isinstance(plugin, TablePlugin):
            result = plugin.infer_table_schema(resource, **options)
            if result is not None:
                return result

    table = load_table(resource, denormalized=True)
    if table is None:
        return None

    return infer_table_schema_from_table(table, **options)
