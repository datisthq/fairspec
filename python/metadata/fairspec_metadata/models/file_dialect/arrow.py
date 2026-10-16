from __future__ import annotations

from typing import Literal

from .base import BaseFileDialect


class ArrowFileDialect(BaseFileDialect):
    format: Literal["arrow"] = "arrow"
