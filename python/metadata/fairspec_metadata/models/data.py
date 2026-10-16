from __future__ import annotations

from typing import Any, Union

from .path import Path

ResourceDataPath = Union[Path, list[Path]]

ResourceDataValue = Union[
    dict[str, Any],
    list[dict[str, Any]],
]

ResourceData = Union[ResourceDataPath, ResourceDataValue]

Data = Any
