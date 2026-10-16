from __future__ import annotations

from typing import Annotated, Literal, Union

from pydantic import Field

from fairspec_metadata.models.column.column import ColumnType
from .base import BaseError


class BaseCellError(BaseError):
    columnName: str = Field(description="The name of the column")
    rowNumber: float = Field(description="The row number where the error occurred")
    cell: str = Field(description="The cell value that caused the error")


class CellTypeError(BaseCellError):
    type: Literal["cell/type"] = Field(description="Error type identifier")
    columnType: ColumnType = Field(description="The expected column type")


class CellMissingError(BaseCellError):
    type: Literal["cell/missing"] = Field(description="Error type identifier")


class CellMinimumError(BaseCellError):
    type: Literal["cell/minimum"] = Field(description="Error type identifier")
    minimum: str = Field(description="The minimum value allowed")


class CellMaximumError(BaseCellError):
    type: Literal["cell/maximum"] = Field(description="Error type identifier")
    maximum: str = Field(description="The maximum value allowed")


class CellExclusiveMinimumError(BaseCellError):
    type: Literal["cell/exclusiveMinimum"] = Field(description="Error type identifier")
    minimum: str = Field(description="The exclusive minimum value")


class CellExclusiveMaximumError(BaseCellError):
    type: Literal["cell/exclusiveMaximum"] = Field(description="Error type identifier")
    maximum: str = Field(description="The exclusive maximum value")


class CellMultipleOfError(BaseCellError):
    type: Literal["cell/multipleOf"] = Field(description="Error type identifier")
    multipleOf: float = Field(description="The multiple of constraint")


class CellMinLengthError(BaseCellError):
    type: Literal["cell/minLength"] = Field(description="Error type identifier")
    minLength: float = Field(description="The minimum length required")


class CellMaxLengthError(BaseCellError):
    type: Literal["cell/maxLength"] = Field(description="Error type identifier")
    maxLength: float = Field(description="The maximum length allowed")


class CellPatternError(BaseCellError):
    type: Literal["cell/pattern"] = Field(description="Error type identifier")
    pattern: str = Field(description="The pattern that must be matched")


class CellUniqueError(BaseCellError):
    type: Literal["cell/unique"] = Field(description="Error type identifier")


class CellConstError(BaseCellError):
    type: Literal["cell/const"] = Field(description="Error type identifier")
    const: str = Field(description="The allowed value")


class CellEnumError(BaseCellError):
    type: Literal["cell/enum"] = Field(description="Error type identifier")
    enum: list[str] = Field(description="The allowed enumeration values")


class CellJsonError(BaseCellError):
    type: Literal["cell/json"] = Field(description="Error type identifier")
    message: str = Field(description="The JSON schema validation error message")
    jsonPointer: str = Field(description="JSON Pointer to the validation error location")


class CellMinItemsError(BaseCellError):
    type: Literal["cell/minItems"] = Field(description="Error type identifier")
    minItems: float = Field(description="The minimum number of items required")


class CellMaxItemsError(BaseCellError):
    type: Literal["cell/maxItems"] = Field(description="Error type identifier")
    maxItems: float = Field(description="The maximum number of items allowed")


CellError = Annotated[
    Union[
        CellTypeError,
        CellMissingError,
        CellMinimumError,
        CellMaximumError,
        CellExclusiveMinimumError,
        CellExclusiveMaximumError,
        CellMultipleOfError,
        CellMinLengthError,
        CellMaxLengthError,
        CellMinItemsError,
        CellMaxItemsError,
        CellPatternError,
        CellUniqueError,
        CellConstError,
        CellEnumError,
        CellJsonError,
    ],
    Field(discriminator="type"),
]
