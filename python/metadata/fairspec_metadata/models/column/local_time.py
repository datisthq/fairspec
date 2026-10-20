from __future__ import annotations

from typing import Literal

from pydantic import Field

from .base import BaseColumn
from .string import BaseStringColumnProperty


class LocalTimeColumnProperty(BaseStringColumnProperty):
    format: Literal["local-time"] = "local-time"
    temporalFormat: str | None = Field(
        default=None,
        description="An optional string specifying the temporal format pattern without a UTC offset specification",
    )


class LocalTimeColumn(BaseColumn):
    type: Literal["local-time"]
    property: LocalTimeColumnProperty
