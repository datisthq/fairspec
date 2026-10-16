from __future__ import annotations

from typing import Literal

from .base import BaseColumn
from .string import BaseStringColumnProperty


class UrlColumnProperty(BaseStringColumnProperty):
    format: Literal["url"] = "url"


class UrlColumn(BaseColumn):
    type: Literal["url"]
    property: UrlColumnProperty
