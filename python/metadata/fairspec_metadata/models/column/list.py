from __future__ import annotations

from enum import StrEnum
from typing import Literal

from pydantic import Field

from .base import BaseColumn
from .string import BaseStringColumnProperty


class ListItemType(StrEnum):
    string = "string"
    integer = "integer"
    number = "number"
    boolean = "boolean"
    date_time = "date-time"
    date = "date"
    time = "time"


class ListColumnProperty(BaseStringColumnProperty):
    format: Literal["list"] = "list"
    itemType: ListItemType | None = Field(
        default=None,
        description="An optional type for items in a list column",
    )
    delimiter: str | None = Field(
        default=None,
        min_length=1,
        max_length=1,
        description="An optional single character used to delimit items in a list column",
    )
    minItems: int | None = Field(
        default=None,
        ge=0,
        description="An optional minimum length constraint for list values",
    )
    maxItems: int | None = Field(
        default=None,
        ge=0,
        description="An optional maximum length constraint for list values",
    )


class ListColumn(BaseColumn):
    type: Literal["list"]
    property: ListColumnProperty
