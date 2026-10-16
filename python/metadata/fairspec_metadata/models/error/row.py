from __future__ import annotations

from typing import Annotated, Literal, Union

from pydantic import Field

from .base import BaseError


class RowPrimaryKeyError(BaseError):
    type: Literal["row/primaryKey"] = Field(description="Error type identifier")
    rowNumber: float = Field(description="The row number where the error occurred")
    columnNames: list[str] = Field(
        description="Column names involved in the primary key constraint violation"
    )


class RowUniqueKeyError(BaseError):
    type: Literal["row/uniqueKey"] = Field(description="Error type identifier")
    rowNumber: float = Field(description="The row number where the error occurred")
    columnNames: list[str] = Field(
        description="Column names involved in the unique key constraint violation"
    )


RowError = Annotated[
    Union[RowPrimaryKeyError, RowUniqueKeyError],
    Field(discriminator="type"),
]
