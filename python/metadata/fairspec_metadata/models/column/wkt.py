from __future__ import annotations

from typing import Literal

from .base import BaseColumn
from .string import BaseStringColumnProperty


class WktColumnProperty(BaseStringColumnProperty):
    format: Literal["wkt"] = "wkt"


class WktColumn(BaseColumn):
    type: Literal["wkt"]
    property: WktColumnProperty
