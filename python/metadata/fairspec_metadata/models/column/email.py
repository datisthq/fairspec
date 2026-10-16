from __future__ import annotations

from typing import Literal

from .base import BaseColumn
from .string import BaseStringColumnProperty


class EmailColumnProperty(BaseStringColumnProperty):
    format: Literal["email"] = "email"


class EmailColumn(BaseColumn):
    type: Literal["email"]
    property: EmailColumnProperty
