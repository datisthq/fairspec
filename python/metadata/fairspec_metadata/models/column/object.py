from __future__ import annotations

from typing import Any, Literal

from pydantic import Field

from ..base import FairspecModel
from .base import BaseColumn, BaseColumnProperty

ObjectNullablePropertyType = (
    Literal["object"]
    | tuple[Literal["object"], Literal["null"]]
    | tuple[Literal["null"], Literal["object"]]
)


class ObjectMissingValueItem(FairspecModel):
    value: str
    label: str


class BaseObjectColumnProperty(BaseColumnProperty):
    type: ObjectNullablePropertyType = "object"
    enum: list[dict[str, Any]] | None = Field(
        default=None,
        description="An optional array of allowed values for the column",
    )
    const: dict[str, Any] | None = Field(
        default=None,
        description="An optional const that all values must match",
    )
    default: list[dict[str, Any]] | None = Field(
        default=None,
        description="An optional default value for the column",
    )
    examples: list[dict[str, Any]] | None = Field(
        default=None,
        description="An optional array of examples for the column",
    )
    missingValues: list[str | ObjectMissingValueItem] | None = Field(
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
    properties: Any | None = None
    additionalProperties: Any | None = None
    patternProperties: Any | None = None
    propertyNames: Any | None = None
    minProperties: float | None = None
    maxProperties: float | None = None
    dependencies: Any | None = None
    dependentRequired: Any | None = None
    dependentSchemas: Any | None = None
    required: Any | None = None


class ObjectColumnProperty(BaseObjectColumnProperty):
    format: Literal[None] = None


class ObjectColumn(BaseColumn):
    type: Literal["object"]
    property: ObjectColumnProperty
