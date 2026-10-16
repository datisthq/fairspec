from __future__ import annotations

from typing import Literal, Union

from pydantic import Field

from ..base import FairspecModel
from .base import BaseColumn
from .integer import BaseIntegerColumnProperty
from .string import BaseStringColumnProperty


class IntegerCategoryItem(FairspecModel):
    value: int
    label: str


class IntegerCategoricalColumnProperty(BaseIntegerColumnProperty):
    format: Literal["categorical"] = "categorical"
    categories: list[int | IntegerCategoryItem] | None = Field(
        default=None,
        description="An optional array of categorical values with optional labels",
    )
    withOrder: bool | None = Field(
        default=None,
        description="An optional boolean indicating whether the categories are ordered",
    )


class StringCategoryItem(FairspecModel):
    value: str
    label: str


class StringCategoricalColumnProperty(BaseStringColumnProperty):
    format: Literal["categorical"] = "categorical"
    categories: list[str | StringCategoryItem] | None = Field(
        default=None,
        description="An optional array of categorical values with optional labels",
    )
    withOrder: bool | None = Field(
        default=None,
        description="An optional boolean indicating whether the categories are ordered",
    )


class CategoricalColumn(BaseColumn):
    type: Literal["categorical"]
    property: Union[
        StringCategoricalColumnProperty,
        IntegerCategoricalColumnProperty,
    ]
