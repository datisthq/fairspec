from __future__ import annotations

from typing import Annotated

from pydantic import ConfigDict, Field

from .base import FairspecModel


class CatalogDataset(FairspecModel):
    model_config = ConfigDict(populate_by_name=True)

    loc: str = Field(description="The location (URI) of the dataset")
    upd: str = Field(description="The last updated date-time of the dataset")
    del_: bool | None = Field(
        default=None,
        alias="del",
        description="Whether the entry is a tombstone for a removed dataset",
    )


Catalog = Annotated[
    list[CatalogDataset],
    Field(
        description="A catalog is an array of dataset references with their locations and update times"
    ),
]
