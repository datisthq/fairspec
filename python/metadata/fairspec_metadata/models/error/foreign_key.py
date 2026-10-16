from __future__ import annotations

from typing import Literal

from pydantic import Field

from fairspec_metadata.models.foreign_key import ForeignKey
from .base import BaseError


class ForeignKeyError(BaseError):
    type: Literal["foreignKey"] = Field(description="Error type identifier")
    foreignKey: ForeignKey = Field(
        description="The foreign key constraint that was violated"
    )
    cells: list[str] = Field(
        description="The cells that violate the foreign key constraint"
    )
