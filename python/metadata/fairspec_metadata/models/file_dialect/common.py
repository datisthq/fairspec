from __future__ import annotations

from enum import StrEnum
from typing import Literal

HeaderRows = Literal[False] | list[int]

Delimiter = str
LineTerminator = str
QuoteChar = str
NullSequence = str
HeaderJoin = str
CommentRows = list[int]
CommentPrefix = str
ColumnNames = list[str]
JsonPointer = str
SheetNumber = int
SheetName = str
TableName = str


class RowType(StrEnum):
    array = "array"
    object = "object"
