from __future__ import annotations

from typing import Literal

from .base import BaseFileDialect
from .common import (
    ColumnNames,
    CommentPrefix,
    CommentRows,
    HeaderJoin,
    HeaderRows,
    LineTerminator,
    NullSequence,
)


class TsvFileDialect(BaseFileDialect):
    format: Literal["tsv"] = "tsv"
    lineTerminator: LineTerminator | None = None
    nullSequence: NullSequence | None = None
    headerRows: HeaderRows | None = None
    headerJoin: HeaderJoin | None = None
    commentRows: CommentRows | None = None
    commentPrefix: CommentPrefix | None = None
    columnNames: ColumnNames | None = None
