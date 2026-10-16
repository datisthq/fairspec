from __future__ import annotations

from typing import Annotated, Literal, Union

from pydantic import Field

from .base import BaseError


class ResourceMissingError(BaseError):
    type: Literal["resource/missing"] = Field(description="Error type identifier")
    referencingResourceName: str = Field(
        description="The name of the referencing resource"
    )


class ResourceTypeError(BaseError):
    type: Literal["resource/type"] = Field(description="Error type identifier")
    expectedResourceType: Literal["data", "table"] = Field(
        description="The expected data type"
    )
    referencingResourceName: str | None = Field(
        default=None,
        description="The name of the referencing resource",
    )


ResourceError = Annotated[
    Union[ResourceMissingError, ResourceTypeError],
    Field(discriminator="type"),
]
