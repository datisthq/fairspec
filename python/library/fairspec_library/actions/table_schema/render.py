from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_library.system import system

if TYPE_CHECKING:
    from fairspec_metadata import RenderTableSchemaOptions, TableSchema


def render_table_schema_as(
    table_schema: TableSchema, options: RenderTableSchemaOptions
) -> str | None:
    for plugin in system.plugins:
        result = plugin.render_table_schema_as(table_schema, options)
        if result is not None:
            return result

    return None
