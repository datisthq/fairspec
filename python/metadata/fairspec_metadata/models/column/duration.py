from __future__ import annotations

from typing import Literal

from .base import BaseColumn
from .string import BaseStringColumnProperty


class DurationColumnProperty(BaseStringColumnProperty):
    format: Literal["duration"] = "duration"


class DurationColumn(BaseColumn):
    type: Literal["duration"]
    property: DurationColumnProperty
