from __future__ import annotations

from pydantic import Field

from ..base import FairspecModel

from .alternate_identifier import AlternateIdentifiers
from .content_type import ContentTypes
from .contributor import Contributors
from .creator import Creators
from .date import Dates
from .description import Descriptions
from .formats import Formats
from .funding_reference import FundingReferences
from .geo_location import GeoLocations
from .identifier import Doi, DoiPrefix, DoiSuffix
from .language import Language
from .publication_year import PublicationYear
from .publisher import Publisher
from .related_identifier import RelatedIdentifiers
from .related_item import RelatedItems
from .rights import RightsList
from .size import Sizes
from .subject import Subjects
from .title import Titles
from .version import Version


class Datacite(FairspecModel):
    doi: Doi | None = Field(
        default=None,
        description="The Digital Object Identifier (DOI) for the resource",
    )
    prefix: DoiPrefix | None = Field(
        default=None,
        description="The DOI prefix for the resource",
    )
    suffix: DoiSuffix | None = Field(
        default=None,
        description="The DOI suffix for the resource",
    )
    creators: Creators | None = Field(
        default=None,
        description="The main researchers involved in producing the data, or the authors of the publication",
    )
    titles: Titles | None = Field(
        default=None,
        description="A name or title by which a resource is known",
    )
    publisher: Publisher | None = Field(
        default=None,
        description="The entity that holds, archives, publishes, prints, distributes, releases, issues, or produces the resource",
    )
    publicationYear: PublicationYear | None = Field(
        default=None,
        description="The year when the data was or will be made publicly available",
    )
    subjects: Subjects | None = Field(
        default=None,
        description="Subject, keywords, classification codes, or key phrases describing the resource",
    )
    contributors: Contributors | None = Field(
        default=None,
        description="The institution or person responsible for collecting, managing, distributing, or otherwise contributing to the development of the resource",
    )
    dates: Dates | None = Field(
        default=None,
        description="Different dates relevant to the work",
    )
    language: Language | None = Field(
        default=None,
        description="The primary language of the resource",
    )
    types: ContentTypes | None = Field(
        default=None,
        description="The type of the resource",
    )
    alternateIdentifiers: AlternateIdentifiers | None = Field(
        default=None,
        description="An identifier or identifiers other than the primary Identifier applied to the resource",
    )
    relatedIdentifiers: RelatedIdentifiers | None = Field(
        default=None,
        description="Identifiers of related resources",
    )
    sizes: Sizes | None = Field(
        default=None,
        description="Size information about the resource",
    )
    formats: Formats | None = Field(
        default=None,
        description="Technical format of the resource",
    )
    version: Version | None = Field(
        default=None,
        description="The version number of the resource",
    )
    rightsList: RightsList | None = Field(
        default=None,
        description="Rights information for this resource",
    )
    descriptions: Descriptions | None = Field(
        default=None,
        description="All additional information that does not fit in any of the other categories",
    )
    geoLocations: GeoLocations | None = Field(
        default=None,
        description="Spatial region or named place where the data was gathered or about which the data is focused",
    )
    fundingReferences: FundingReferences | None = Field(
        default=None,
        description="Information about financial support (funding) for the resource",
    )
    relatedItems: RelatedItems | None = Field(
        default=None,
        description="Information about resources related to the one being registered",
    )
