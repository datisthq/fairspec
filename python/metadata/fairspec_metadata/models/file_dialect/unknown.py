from __future__ import annotations

from .base import BaseFileDialect


class UnknownFileDialect(BaseFileDialect):
    format: str | None = None
