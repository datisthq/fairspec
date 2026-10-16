from __future__ import annotations

from dataclasses import dataclass
from typing import TypedDict

import polars as pl
from fairspec_metadata import Column


@dataclass
class PolarsColumn:
    name: str
    type: type[pl.DataType]


@dataclass
class ColumnMapping:
    source: PolarsColumn
    target: Column


class DenormalizeColumnOptions(TypedDict, total=False):
    nativeTypes: list[str]
