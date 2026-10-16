from __future__ import annotations

from typing import Literal

from pydantic import Field

from ..base import FairspecModel
from .base import BaseColumn, BaseColumnProperty

NumberNullablePropertyType = (
    Literal["number"]
    | tuple[Literal["number"], Literal["null"]]
    | tuple[Literal["null"], Literal["number"]]
)


class NumberMissingValueItem(FairspecModel):
    value: str | int
    label: str


class BaseNumberColumnProperty(BaseColumnProperty):
    type: NumberNullablePropertyType = "number"
    enum: list[float] | None = Field(
        default=None,
        description="An optional array of allowed values for the column",
    )
    const: float | None = Field(
        default=None,
        description="An optional const that all values must match",
    )
    default: list[float] | None = Field(
        default=None,
        description="An optional default value for the column",
    )
    examples: list[float] | None = Field(
        default=None,
        description="An optional array of examples for the column",
    )
    missingValues: list[str | int | NumberMissingValueItem] | None = Field(
        default=None,
        description="An optional column-specific list of values that represent missing or null data",
    )
    minimum: float | None = Field(
        default=None,
        description="An optional minimum value constraint (inclusive)",
    )
    maximum: float | None = Field(
        default=None,
        description="An optional maximum value constraint (inclusive)",
    )
    exclusiveMinimum: float | None = Field(
        default=None,
        description="An optional minimum value constraint (exclusive)",
    )
    exclusiveMaximum: float | None = Field(
        default=None,
        description="An optional maximum value constraint (exclusive)",
    )
    multipleOf: float | None = Field(
        default=None,
        gt=0,
        description="An optional constraint that values must be a multiple of this number",
    )
    decimalChar: str | None = Field(
        default=None,
        min_length=1,
        max_length=1,
        description="An optional single character used as the decimal separator in the data",
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


class NumberColumnProperty(BaseNumberColumnProperty):
    format: Literal[None] = None


class NumberColumn(BaseColumn):
    type: Literal["number"]
    property: NumberColumnProperty
