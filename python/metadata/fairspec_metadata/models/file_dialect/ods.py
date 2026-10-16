from __future__ import annotations

from typing import Literal

from .base import BaseFileDialect
from .common import (
    ColumnNames,
    CommentPrefix,
    CommentRows,
    HeaderJoin,
    HeaderRows,
    SheetName,
    SheetNumber,
)


class OdsFileDialect(BaseFileDialect):
    format: Literal["ods"] = "ods"
    sheetName: SheetName | None = None
    sheetNumber: SheetNumber | None = None
    headerRows: HeaderRows | None = None
    headerJoin: HeaderJoin | None = None
    commentRows: CommentRows | None = None
    commentPrefix: CommentPrefix | None = None
    columnNames: ColumnNames | None = None
