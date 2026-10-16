from __future__ import annotations

from typing import TypedDict


class ConcurrencyOptions(TypedDict, total=False):
    concurrency: int
