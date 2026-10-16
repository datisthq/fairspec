from __future__ import annotations

from typing import Literal

from pydantic import Field

from .base import BaseError


class DataError(BaseError):
    type: Literal["data"] = Field(description="Error type identifier")
    message: str = Field(description="The JSON parsing error message")
    jsonPointer: str = Field(description="JSON Pointer to the location of the error")
