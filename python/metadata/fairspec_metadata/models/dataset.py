from __future__ import annotations

from pydantic import ConfigDict, Field

from .base import FairspecModel

from .datacite.datacite import Datacite
from .resource import Resource


class Dataset(Datacite):
    model_config = ConfigDict(populate_by_name=True)

    profile: str | None = Field(
        default=None,
        alias="$schema",
        description="Fairspec Dataset profile url.",
    )
    resources: list[Resource] | None = Field(
        default=None,
        description="A list of resources. Each item must be a Resource object describing data files or inline data.",
    )


class RenderDatasetOptions(FairspecModel):
    format: str


class ConvertDatasetToOptions(FairspecModel):
    format: str


class ConvertDatasetFromOptions(FairspecModel):
    format: str
