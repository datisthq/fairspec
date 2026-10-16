from __future__ import annotations

from typing import TypeGuard


def get_is_object(value: object) -> TypeGuard[dict[str, object]]:
    return isinstance(value, dict)
