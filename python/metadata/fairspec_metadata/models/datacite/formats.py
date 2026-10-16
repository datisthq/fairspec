from __future__ import annotations

from typing import Annotated

from pydantic import Field

Formats = Annotated[
    list[str],
    Field(
        description="Technical format of the resource (e.g., file format, physical medium, or dimensions of the resource)"
    ),
]
