from __future__ import annotations

from typing import Literal

from .base import BaseColumn
from .string import BaseStringColumnProperty


class WkbColumnProperty(BaseStringColumnProperty):
    format: Literal["wkb"] = "wkb"


class WkbColumn(BaseColumn):
    type: Literal["wkb"]
    property: WkbColumnProperty
