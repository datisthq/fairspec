from __future__ import annotations

from typing import Annotated

from pydantic import Field

from .base import FairspecModel


class CatalogDataset(FairspecModel):
    loc: str = Field(description="The location (URI) of the dataset")
    upd: str = Field(description="The last updated date-time of the dataset")


Catalog = Annotated[
    list[CatalogDataset],
    Field(
        description="A catalog is an array of dataset references with their locations and update times"
    ),
]
