from __future__ import annotations

from pydantic import Field

from ..base import FairspecModel

from .common import ContentTypeGeneral


class ContentTypes(FairspecModel):
    resourceType: str | None = Field(
        default=None,
        description="A description of the resource (free text)",
    )
    resourceTypeGeneral: ContentTypeGeneral = Field(
        description="The general type of the resource (e.g., Dataset, Software, Collection, Image, etc.)"
    )
