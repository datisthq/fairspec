from __future__ import annotations

from pydantic import ConfigDict, Field

from ..base import FairspecModel


class BaseFileDialect(FairspecModel):
    model_config = ConfigDict(populate_by_name=True)

    profile: str | None = Field(
        default=None,
        alias="$schema",
        description="Fairspec Dialect profile url.",
    )
    title: str | None = Field(
        default=None,
        description="An optional human-readable title of the format",
    )
    description: str | None = Field(
        default=None,
        description="An optional detailed description of the format",
    )
