from __future__ import annotations

from typing import Literal

from pydantic import Field

from .base import BaseColumn
from .string import BaseStringColumnProperty


class DateColumnProperty(BaseStringColumnProperty):
    format: Literal["date"] = "date"
    temporalFormat: str | None = Field(
        default=None,
        description="An optional string specifying the temporal format pattern",
    )


class DateColumn(BaseColumn):
    type: Literal["date"]
    property: DateColumnProperty
