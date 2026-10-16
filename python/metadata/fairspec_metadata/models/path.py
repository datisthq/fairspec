from __future__ import annotations

from typing import Annotated

from pydantic import Field

InternalPath = Annotated[
    str,
    Field(
        pattern=r"^[^./~\\:][^./\\:]*(\.[^./\\:]+)*\.?(/([^./\\:]+(\.[^./\\:]+)*\.?|\.([^./\\:]+(\.[^./\\:]+)*\.?)?))*$"
    ),
]

ExternalPath = Annotated[str, Field(pattern=r"^https?://")]

Path = InternalPath | ExternalPath
