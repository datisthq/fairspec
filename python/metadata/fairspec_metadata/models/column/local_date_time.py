from __future__ import annotations

from typing import Literal

from pydantic import Field

from .base import BaseColumn
from .string import BaseStringColumnProperty


class LocalDateTimeColumnProperty(BaseStringColumnProperty):
    format: Literal["local-date-time"] = "local-date-time"
    temporalFormat: str | None = Field(
        default=None,
        description="An optional string specifying the temporal format pattern without a UTC offset specification",
    )


class LocalDateTimeColumn(BaseColumn):
    type: Literal["local-date-time"]
    property: LocalDateTimeColumnProperty
