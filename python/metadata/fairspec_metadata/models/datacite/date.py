from __future__ import annotations

from typing import Annotated

from pydantic import Field

from ..base import FairspecModel

from .common import DateType

DateValue = Annotated[
    str,
    Field(
        description="Different date formats are supported: YYYY, YYYY-MM, YYYY-MM-DD, YYYY-MM-DDThh:mm:ss, YYYY-MM-DDThh:mm:ssTZD, or any of these formats with ranges separated by /"
    ),
]


class DataciteDate(FairspecModel):
    date: DateValue = Field(
        description="The date associated with an event in the lifecycle of the resource"
    )
    dateType: DateType = Field(
        description="The type of date (e.g., Accepted, Available, Created, Issued, Submitted, Updated, etc.)"
    )
    dateInformation: str | None = Field(
        default=None,
        description="Additional information about the date",
    )


Dates = Annotated[
    list[DataciteDate],
    Field(description="Different dates relevant to the work"),
]
