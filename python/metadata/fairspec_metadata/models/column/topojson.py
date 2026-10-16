from __future__ import annotations

from typing import Literal

from .base import BaseColumn
from .object import BaseObjectColumnProperty


class TopojsonColumnProperty(BaseObjectColumnProperty):
    format: Literal["topojson"] = "topojson"


class TopojsonColumn(BaseColumn):
    type: Literal["topojson"]
    property: TopojsonColumnProperty
