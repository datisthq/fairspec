from __future__ import annotations

from typing import Annotated, Literal

from pydantic import AfterValidator, Field

from ..base import FairspecModel
from .base import BaseColumn, BaseColumnProperty, create_type_array_check

IntegerNullablePropertyType = (
    Literal["integer"]
    | Annotated[
        tuple[Literal["integer", "null"], ...],
        AfterValidator(create_type_array_check("integer")),
    ]
)


class IntegerMissingValueItem(FairspecModel):
    value: str | int
    label: str


class BaseIntegerColumnProperty(BaseColumnProperty):
    type: IntegerNullablePropertyType = "integer"
    enum: list[int | None] | None = Field(
        default=None,
        description="An optional array of allowed values for the column",
    )
    const: int | None = Field(
        default=None,
        description="An optional const that all values must match",
    )
    default: int | None = Field(
        default=None,
        description="An optional default value for the column",
    )
    examples: list[int | None] | None = Field(
        default=None,
        description="An optional array of examples for the column",
    )
    missingValues: list[str | int | IntegerMissingValueItem] | None = Field(
        default=None,
        description="An optional column-specific list of values that represent missing or null data",
    )
    minimum: int | None = Field(
        default=None,
        description="An optional minimum value constraint (inclusive)",
    )
    maximum: int | None = Field(
        default=None,
        description="An optional maximum value constraint (inclusive)",
    )
    exclusiveMinimum: int | None = Field(
        default=None,
        description="An optional minimum value constraint (exclusive)",
    )
    exclusiveMaximum: int | None = Field(
        default=None,
        description="An optional maximum value constraint (exclusive)",
    )
    multipleOf: int | None = Field(
        default=None,
        ge=1,
        description="An optional constraint that values must be a multiple of this number",
    )
    groupChar: str | None = Field(
        default=None,
        min_length=1,
        max_length=1,
        description="An optional single character used as the thousands separator in the data",
    )
    withText: bool | None = Field(
        default=None,
        description="An optional boolean indicating whether numeric values may include non-numeric text that should be stripped during parsing",
    )


class IntegerColumnProperty(BaseIntegerColumnProperty):
    format: str | None = None


class IntegerColumn(BaseColumn):
    type: Literal["integer"]
    property: IntegerColumnProperty
