from __future__ import annotations

from pydantic import Field

from .base import FairspecModel
from .error.error import FairspecError


class Report(FairspecModel):
    valid: bool = Field(description="Whether the validation passed without errors")
    errors: list[FairspecError] = Field(
        description="Array of validation errors encountered"
    )
