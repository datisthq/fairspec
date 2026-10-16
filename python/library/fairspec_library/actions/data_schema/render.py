from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_library.system import system

if TYPE_CHECKING:
    from fairspec_metadata import DataSchema, RenderDataSchemaOptions


def render_data_schema_as(
    data_schema: DataSchema, options: RenderDataSchemaOptions
) -> str | None:
    for plugin in system.plugins:
        result = plugin.render_data_schema_as(data_schema, options)
        if result is not None:
            return result

    return None
