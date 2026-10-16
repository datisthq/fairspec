from __future__ import annotations

from typing import Annotated, Self

from pydantic import Field, model_validator

from ..base import FairspecModel

from .common import ContentTypeGeneral, RelatedIdentifierType, RelationType


class RelatedObject(FairspecModel):
    relationType: RelationType = Field(
        description="Description of the relationship of the resource being registered and the related resource"
    )
    relatedMetadataScheme: str | None = Field(
        default=None,
        description="The name of the scheme (only for HasMetadata/IsMetadataFor relations)",
    )
    schemeUri: str | None = Field(
        default=None,
        description="The URI of the relatedMetadataScheme (only for HasMetadata/IsMetadataFor relations)",
    )
    schemeType: str | None = Field(
        default=None,
        description="The type of the relatedMetadataScheme (only for HasMetadata/IsMetadataFor relations)",
    )
    resourceTypeGeneral: ContentTypeGeneral | None = Field(
        default=None,
        description="The general type of the related resource",
    )


class RelatedIdentifier(RelatedObject):
    relatedIdentifier: str = Field(description="Identifiers of related resources")
    relatedIdentifierType: RelatedIdentifierType = Field(
        description="The type of the RelatedIdentifier (e.g., DOI, Handle, URL, etc.)"
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


RelatedIdentifiers = Annotated[
    list[RelatedIdentifier],
    Field(description="Identifiers of related resources"),
]
