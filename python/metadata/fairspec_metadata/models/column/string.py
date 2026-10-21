from __future__ import annotations

from typing import Annotated, Literal

from pydantic import AfterValidator, Field

from ..base import FairspecModel
from .base import BaseColumn, BaseColumnProperty, create_type_array_check

StringNullablePropertyType = (
    Literal["string"]
    | Annotated[
        tuple[Literal["string", "null"], ...],
        AfterValidator(create_type_array_check("string")),
    ]
)


class StringMissingValueItem(FairspecModel):
    value: str
    label: str


class BaseStringColumnProperty(BaseColumnProperty):
    type: StringNullablePropertyType = "string"
    enum: list[str | None] | None = Field(
        default=None,
        description="An optional array of allowed values for the column",
    )
    const: str | None = Field(
        default=None,
        description="An optional const that all values must match",
    )
    default: str | None = Field(
        default=None,
        description="An optional default value for the column",
    )
    examples: list[str | None] | None = Field(
        default=None,
        description="An optional array of examples for the column",
    )
    missingValues: list[str | StringMissingValueItem] | None = Field(
        default=None,
        description="An optional column-specific list of values that represent missing or null data",
    )
    minLength: int | None = Field(
        default=None,
        ge=0,
        description="An optional minimum length constraint for string values",
    )
    maxLength: int | None = Field(
        default=None,
        ge=0,
        description="An optional maximum length constraint for string values",
    )
    pattern: str | None = Field(
        default=None,
        description="An optional regular expression pattern that values must match",
    )


class StringColumnProperty(BaseStringColumnProperty):
    format: str | None = None


class StringColumn(BaseColumn):
    type: Literal["string"]
    property: StringColumnProperty
