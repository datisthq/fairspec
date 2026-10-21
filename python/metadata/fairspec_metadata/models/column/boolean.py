from __future__ import annotations

from typing import Annotated, Literal

from pydantic import AfterValidator, Field

from ..base import FairspecModel
from .base import BaseColumn, BaseColumnProperty, create_type_array_check

BooleanNullablePropertyType = (
    Literal["boolean"]
    | Annotated[
        tuple[Literal["boolean", "null"], ...],
        AfterValidator(create_type_array_check("boolean")),
    ]
)


class StringIntMissingValue(FairspecModel):
    value: str | int
    label: str


class BooleanColumnProperty(BaseColumnProperty):
    type: BooleanNullablePropertyType = "boolean"
    format: str | None = None
    enum: list[bool | None] | None = Field(
        default=None,
        description="An optional array of allowed values for the column",
    )
    const: bool | None = Field(
        default=None,
        description="An optional const that all values must match",
    )
    default: bool | None = Field(
        default=None,
        description="An optional default value for the column",
    )
    examples: list[bool | None] | None = Field(
        default=None,
        description="An optional array of examples for the column",
    )
    missingValues: list[str | int | StringIntMissingValue] | None = Field(
        default=None,
        description="An optional column-specific list of values that represent missing or null data",
    )
    trueValues: list[str] | None = Field(
        default=None,
        description="An optional array of string values that should be interpreted as true when parsing data",
    )
    falseValues: list[str] | None = Field(
        default=None,
        description="An optional array of string values that should be interpreted as false when parsing data",
    )


class BooleanColumn(BaseColumn):
    type: Literal["boolean"]
    property: BooleanColumnProperty
