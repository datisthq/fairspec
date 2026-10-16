from __future__ import annotations

from typing import Required

import polars as pl
from fairspec_dataset import InferFileDialectOptions

from .schema import InferTableSchemaOptions, TableSchemaOptions

Table = pl.LazyFrame


class LoadTableOptions(InferFileDialectOptions, InferTableSchemaOptions, total=False):
    previewBytes: int
    denormalized: bool


class SaveTableOptions(TableSchemaOptions, total=False):
    path: Required[str]
    fileDialect: object
    tableSchema: object
    overwrite: bool
