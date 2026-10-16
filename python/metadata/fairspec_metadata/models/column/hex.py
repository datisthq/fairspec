from __future__ import annotations

from typing import Literal

from .base import BaseColumn
from .string import BaseStringColumnProperty


class HexColumnProperty(BaseStringColumnProperty):
    format: Literal["hex"] = "hex"


class HexColumn(BaseColumn):
    type: Literal["hex"]
    property: HexColumnProperty
