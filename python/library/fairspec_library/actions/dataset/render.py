from __future__ import annotations

from typing import TYPE_CHECKING

from fairspec_library.system import system

if TYPE_CHECKING:
    from fairspec_metadata import Dataset, RenderDatasetOptions


def render_dataset_as(dataset: Dataset, options: RenderDatasetOptions) -> str | None:
    for plugin in system.plugins:
        result = plugin.render_dataset_as(dataset, options)
        if result is not None:
            return result

    return None
