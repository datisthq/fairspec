from __future__ import annotations

from typing import Literal

from pydantic import Field

from .base import BaseColumn
from .string import BaseStringColumnProperty


class TimeColumnProperty(BaseStringColumnProperty):
    format: Literal["time"] = "time"
    temporalFormat: str | None = Field(
        default=None,
        description="An optional string specifying the temporal format pattern with a UTC offset specification",
    )


class TimeColumn(BaseColumn):
    type: Literal["time"]
    property: TimeColumnProperty
