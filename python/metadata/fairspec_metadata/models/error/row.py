from __future__ import annotations

from typing import Annotated, Literal, Union

from pydantic import Field

from .base import BaseError


class RowPrimaryKeyError(BaseError):
    type: Literal["row/primaryKey"] = Field(description="Error type identifier")
    rowNumber: int = Field(description="The row number where the error occurred")
    columnNames: list[str] = Field(
        description="Column names involved in the primary key constraint violation"
    )


class RowUniqueKeyError(BaseError):
    type: Literal["row/uniqueKey"] = Field(description="Error type identifier")
    rowNumber: int = Field(description="The row number where the error occurred")
    columnNames: list[str] = Field(
        description="Column names involved in the unique key constraint violation"
    )


class RowCheckError(BaseError):
    type: Literal["row/check"] = Field(description="Error type identifier")
    rowNumber: int = Field(description="The row number where the error occurred")
    checkName: str = Field(description="The name of the check that failed")
    message: str | None = Field(
        default=None, description="The message of the check that failed"
    )


RowError = Annotated[
    Union[RowPrimaryKeyError, RowUniqueKeyError, RowCheckError],
    Field(discriminator="type"),
]
