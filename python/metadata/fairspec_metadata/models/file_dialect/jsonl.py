from __future__ import annotations

from typing import Literal

from .base import BaseFileDialect
from .common import (
    ColumnNames,
    CommentPrefix,
    CommentRows,
    HeaderJoin,
    HeaderRows,
    RowType,
)


class JsonlFileDialect(BaseFileDialect):
    format: Literal["jsonl"] = "jsonl"
    rowType: RowType | None = None
    headerRows: HeaderRows | None = None
    headerJoin: HeaderJoin | None = None
    commentRows: CommentRows | None = None
    commentPrefix: CommentPrefix | None = None
    columnNames: ColumnNames | None = None
