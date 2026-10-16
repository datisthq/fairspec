from __future__ import annotations

from typing import Annotated, Literal, Union

from pydantic import Field

from fairspec_metadata.models.column.column import ColumnType
from .base import BaseError


class ColumnMissingError(BaseError):
    type: Literal["column/missing"] = Field(description="Error type identifier")
    columnName: str = Field(description="Names of missing column")


class ColumnTypeError(BaseError):
    type: Literal["column/type"] = Field(description="Error type identifier")
    columnName: str = Field(description="The name of the column")
    expectedColumnType: ColumnType = Field(
        description="The column type that was expected"
    )
    actualColumnType: ColumnType = Field(description="The actual column type found")


ColumnError = Annotated[
    Union[ColumnMissingError, ColumnTypeError],
    Field(discriminator="type"),
]
