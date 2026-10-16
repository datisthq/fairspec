from __future__ import annotations

from typing import Annotated

from pydantic import Field

Sizes = Annotated[
    list[str],
    Field(
        description="Unstructured size information about the resource (e.g., '15 pages', '6 MB')"
    ),
]
