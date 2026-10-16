from __future__ import annotations

from typing import Annotated, Self

from pydantic import Field, model_validator

from ..base import FairspecModel

from .common import (
    ContentTypeGeneral,
    NumberType,
    RelatedIdentifierType,
)
from .contributor import Contributors
from .creator import Creators
from .publication_year import PublicationYear
from .related_identifier import RelatedObject
from .title import Titles


class RelatedItemIdentifier(FairspecModel):
    relatedItemIdentifier: str = Field(description="Identifier for the related item")
    relatedItemIdentifierType: RelatedIdentifierType = Field(
        description="The type of the RelatedItemIdentifier"
    )


class RelatedItem(RelatedObject):
    relatedItemIdentifier: RelatedItemIdentifier | None = Field(
        default=None,
        description="Identifiers of related items",
    )
    relatedItemType: ContentTypeGeneral = Field(
        description="The general type of the related item"
    )
    creators: Creators | None = None
    contributors: Contributors | None = None
    titles: Titles = Field(description="The title(s) of the related item")
    publicationYear: PublicationYear | None = None
    volume: str | None = Field(
        default=None,
        description="Volume of the related item",
    )
    issue: str | None = Field(
        default=None,
        description="Issue of the related item",
    )
    firstPage: str | None = Field(
        default=None,
        description="First page of the related item",
    )
    lastPage: str | None = Field(
        default=None,
        description="Last page of the related item",
    )
    edition: str | None = Field(
        default=None,
        description="Edition of the related item",
    )
    publisher: str | None = Field(
        default=None,
        description="Publisher of the related item",
    )
    number: str | None = Field(
        default=None,
        description="Number of the related item (e.g., report number, article number)",
    )
    numberType: NumberType | None = Field(
        default=None,
        description="The type of the number",
    )

    @model_validator(mode="after")
    def validate_metadata_fields(self) -> Self:
        has_metadata_relation = self.relationType in (
            "HasMetadata",
            "IsMetadataFor",
        )
        if not has_metadata_relation:
            if (
                self.relatedMetadataScheme is not None
                or self.schemeUri is not None
                or self.schemeType is not None
            ):
                raise ValueError(
                    "relatedMetadataScheme, schemeUri, and schemeType are only allowed for HasMetadata/IsMetadataFor relations"
                )
        return self


RelatedItems = Annotated[
    list[RelatedItem],
    Field(description="Information about a resource related to the one being registered"),
]
