from __future__ import annotations

from typing import Literal

from pydantic import Field

from .base import BaseColumn
from .string import BaseStringColumnProperty


class DecimalColumnProperty(BaseStringColumnProperty):
    format: Literal["decimal"] = "decimal"
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


class DecimalColumn(BaseColumn):
    type: Literal["decimal"]
    property: DecimalColumnProperty
