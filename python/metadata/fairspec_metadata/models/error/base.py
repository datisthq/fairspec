from __future__ import annotations

from pydantic import Field

from ..base import FairspecModel


class BaseError(FairspecModel):
    type: str = Field(description="Error type identifier")
    resourceName: str | None = Field(
        default=None,
        description="Name of the resource if available",
    )
