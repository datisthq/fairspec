from __future__ import annotations

from typing import TYPE_CHECKING, Unpack

from fairspec_dataset import DatasetPlugin, InferFileDialectOptions

from fairspec_library.system import system

if TYPE_CHECKING:
    from fairspec_metadata import FileDialect, Resource


def infer_file_dialect(
    resource: Resource, **options: Unpack[InferFileDialectOptions]
) -> FileDialect | None:
    for plugin in system.plugins:
        if isinstance(plugin, DatasetPlugin):
            result = plugin.infer_file_dialect(resource, **options)
            if result is not None:
                return result

    return None
