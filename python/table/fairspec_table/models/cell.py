from __future__ import annotations

from dataclasses import dataclass

import polars as pl


@dataclass
class CellMapping:
    source: pl.Expr
    target: pl.Expr
