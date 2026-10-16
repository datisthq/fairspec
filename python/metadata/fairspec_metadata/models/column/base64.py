from __future__ import annotations

from typing import Literal

from .base import BaseColumn
from .string import BaseStringColumnProperty


class Base64ColumnProperty(BaseStringColumnProperty):
    format: Literal["base64"] = "base64"


class Base64Column(BaseColumn):
    type: Literal["base64"]
    property: Base64ColumnProperty
