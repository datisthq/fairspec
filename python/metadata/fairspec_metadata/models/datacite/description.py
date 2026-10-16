from __future__ import annotations

from typing import Annotated

from pydantic import Field

from ..base import FairspecModel

from .common import DescriptionType


class DataciteDescription(FairspecModel):
    description: str = Field(
        description="All additional information that does not fit in any of the other categories"
    )
    descriptionType: DescriptionType = Field(
        description="The type of the Description (e.g., Abstract, Methods, TechnicalInfo, etc.)"
    )
    lang: str | None = Field(
        default=None,
        description="Language of the description, specified using ISO 639-1 or ISO 639-3 codes",
    )


Descriptions = Annotated[
    list[DataciteDescription],
    Field(
        description="All additional information that does not fit in any of the other categories"
    ),
]
