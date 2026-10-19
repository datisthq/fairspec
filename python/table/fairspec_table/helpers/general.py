from __future__ import annotations

from typing import TypeGuard


def get_is_object(value: object) -> TypeGuard[dict[str, object]]:
    return isinstance(value, dict)


# A bound validates as a float, so 10 would otherwise render as 10.0
def stringify_number(value: float) -> str:
    if isinstance(value, float) and value.is_integer():
        return str(int(value))
    return str(value)
