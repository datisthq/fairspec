from __future__ import annotations

from typing import Any


def get_is_descriptor(value: Any) -> bool:
    return isinstance(value, dict)
