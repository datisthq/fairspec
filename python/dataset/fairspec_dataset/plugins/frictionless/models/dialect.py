from __future__ import annotations

from typing import Annotated, Literal

from pydantic import Field

from .base import FrictionlessModel

RowNumber = Annotated[int, Field(ge=1)]


class FrictionlessDialect(FrictionlessModel):
    header: bool | None = None
    headerRows: list[RowNumber] | None = None
    headerJoin: str | None = None
    commentRows: list[RowNumber] | None = None
    commentChar: str | None = None
    delimiter: str | None = None
    lineTerminator: str | None = None
    quoteChar: str | None = None
    nullSequence: str | None = None
    property: str | None = None
    itemType: Literal["array", "object"] | None = None
    itemKeys: list[str] | None = None
    sheetNumber: int | None = None
    sheetName: str | None = None
    table: str | None = None
