from __future__ import annotations

from typing import Annotated

from pydantic import Field

PublicationYear = Annotated[
    str,
    Field(
        pattern=r"^[0-9]{4}$",
        description="The year when the data was or will be made publicly available in YYYY format",
    ),
]
