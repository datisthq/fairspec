from __future__ import annotations

from typing import Literal

from .base import BaseColumn
from .object import BaseObjectColumnProperty


class GeojsonColumnProperty(BaseObjectColumnProperty):
    format: Literal["geojson"] = "geojson"


class GeojsonColumn(BaseColumn):
    type: Literal["geojson"]
    property: GeojsonColumnProperty
