from __future__ import annotations

from typing import Literal

from pydantic import Field

from .base import BaseColumn
from .string import BaseStringColumnProperty


class DateTimeColumnProperty(BaseStringColumnProperty):
    format: Literal["date-time"] = "date-time"
    temporalFormat: str | None = Field(
        default=None,
        description="An optional string specifying the temporal format pattern with a UTC offset specification",
    )


class DateTimeColumn(BaseColumn):
    type: Literal["date-time"]
    property: DateTimeColumnProperty
