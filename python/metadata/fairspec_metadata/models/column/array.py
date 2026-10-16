from __future__ import annotations

from typing import Any, Literal

from pydantic import Field

from ..base import FairspecModel
from .base import BaseColumn, BaseColumnProperty

ArrayNullablePropertyType = (
    Literal["array"]
    | tuple[Literal["array"], Literal["null"]]
    | tuple[Literal["null"], Literal["array"]]
)


class ArrayMissingValueItem(FairspecModel):
    value: str
    label: str


class ArrayColumnProperty(BaseColumnProperty):
    type: ArrayNullablePropertyType = "array"
    format: Literal[None] = None
    enum: list[Any] | None = Field(
        default=None,
        description="An optional array of allowed values for the column",
    )
    const: list[Any] | None = Field(
        default=None,
        description="An optional const that all values must match",
    )
    default: list[list[Any]] | None = Field(
        default=None,
        description="An optional default value for the column",
    )
    examples: list[list[Any]] | None = Field(
        default=None,
        description="An optional array of examples for the column",
    )
    missingValues: list[str | ArrayMissingValueItem] | None = Field(
        default=None,
        description="An optional column-specific list of values that represent missing or null data",
    )
    allOf: Any | None = None
    anyOf: Any | None = None
    oneOf: Any | None = None
    not_: Any | None = Field(default=None, alias="not")
    if_: Any | None = Field(default=None, alias="if")
    then: Any | None = None
    else_: Any | None = Field(default=None, alias="else")
    items: Any | None = None
    prefixItems: Any | None = None
    additionalItems: Any | None = None
    contains: Any | None = None
    minContains: float | None = None
    maxContains: float | None = None
    maxItems: float | None = None
    minItems: float | None = None
    uniqueItems: bool | None = None


class ArrayColumn(BaseColumn):
    type: Literal["array"]
    property: ArrayColumnProperty
