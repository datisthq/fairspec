from __future__ import annotations

from typing import Annotated

from pydantic import Field

Language = Annotated[
    str,
    Field(
        description="The primary language of the resource. Allowed values are taken from IETF BCP 47, ISO 639-1 language code"
    ),
]
