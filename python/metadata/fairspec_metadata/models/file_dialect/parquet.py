from __future__ import annotations

from typing import Literal

from .base import BaseFileDialect


class ParquetFileDialect(BaseFileDialect):
    format: Literal["parquet"] = "parquet"
