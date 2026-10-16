from __future__ import annotations

from typing import Annotated

from pydantic import Field

UniqueKey = Annotated[
    list[str],
    Field(
        min_length=1,
        description="An array of column names whose combined values must be unique",
    ),
]
