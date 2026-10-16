from __future__ import annotations

from typing import Literal

from .base import BaseFileDialect
from .common import (
    ColumnNames,
    CommentPrefix,
    CommentRows,
    Delimiter,
    HeaderJoin,
    HeaderRows,
    LineTerminator,
    NullSequence,
    QuoteChar,
)


class CsvFileDialect(BaseFileDialect):
    format: Literal["csv"] = "csv"
    delimiter: Delimiter | None = None
    lineTerminator: LineTerminator | None = None
    quoteChar: QuoteChar | None = None
    nullSequence: NullSequence | None = None
    headerRows: HeaderRows | None = None
    headerJoin: HeaderJoin | None = None
    commentRows: CommentRows | None = None
    commentPrefix: CommentPrefix | None = None
    columnNames: ColumnNames | None = None
