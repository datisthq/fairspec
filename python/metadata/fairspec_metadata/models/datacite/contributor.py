from __future__ import annotations

from typing import Annotated

from pydantic import Field

from .common import ContributorType
from .creator import Creator


class Contributor(Creator):
    contributorType: ContributorType = Field(
        description="The type of contributor (e.g., ContactPerson, DataCollector, Editor, etc.)"
    )


Contributors = Annotated[
    list[Contributor],
    Field(
        description="The institution or person responsible for collecting, managing, distributing, or otherwise contributing to the development of the resource"
    ),
]
