from __future__ import annotations

from typing import Any, Literal

from pydantic import Field

from ..base import FairspecModel
from .base import BaseColumn, BaseColumnProperty


class UnknownMissingValueItem(FairspecModel):
    value: str
    label: str


class UnknownColumnProperty(BaseColumnProperty):
    type: Literal["null"] | None = None
    format: Literal[None] = None
    enum: list[list[Any]] | None = Field(
        default=None,
        description="An optional array of allowed values for the column",
    )
    const: list[Any] | None = Field(
        default=None,
        description="An optional const that all values must match",
    )
    default: list[Any] | None = Field(
        default=None,
        description="An optional default value for the column",
    )
    examples: list[list[Any]] | None = Field(
        default=None,
        description="An optional array of examples for the column",
    )
    missingValues: list[str | UnknownMissingValueItem] | None = Field(
        default=None,
        description="An optional column-specific list of values that represent missing or null data",
    )


class UnknownColumn(BaseColumn):
    type: Literal["unknown"]
    property: UnknownColumnProperty
