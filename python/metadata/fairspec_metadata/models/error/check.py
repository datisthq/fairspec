from __future__ import annotations

from typing import Literal

from pydantic import Field

from .base import BaseError


class TableCheckError(BaseError):
    type: Literal["table/check"] = Field(description="Error type identifier")
    checkName: str = Field(description="The name of the check that failed")
    message: str | None = Field(
        default=None, description="The message of the check that failed"
    )


class CheckError(BaseError):
    type: Literal["check"] = Field(description="Error type identifier")
    checkName: str = Field(description="The name of the check that cannot be evaluated")
    message: str = Field(description="Why the check cannot be evaluated")
