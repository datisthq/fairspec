from __future__ import annotations

from enum import StrEnum
from typing import Annotated

from pydantic import Field


class CreatorNameType(StrEnum):
    Organizational = "Organizational"
    Personal = "Personal"


class TitleType(StrEnum):
    AlternativeTitle = "AlternativeTitle"
    Subtitle = "Subtitle"
    TranslatedTitle = "TranslatedTitle"
    Other = "Other"


class ContributorType(StrEnum):
    ContactPerson = "ContactPerson"
    DataCollector = "DataCollector"
    DataCurator = "DataCurator"
    DataManager = "DataManager"
    Distributor = "Distributor"
    Editor = "Editor"
    HostingInstitution = "HostingInstitution"
    Producer = "Producer"
    ProjectLeader = "ProjectLeader"
    ProjectManager = "ProjectManager"
    ProjectMember = "ProjectMember"
    RegistrationAgency = "RegistrationAgency"
    RegistrationAuthority = "RegistrationAuthority"
    RelatedPerson = "RelatedPerson"
    Researcher = "Researcher"
    ResearchGroup = "ResearchGroup"
    RightsHolder = "RightsHolder"
    Sponsor = "Sponsor"
    Supervisor = "Supervisor"
    Translator = "Translator"
    WorkPackageLeader = "WorkPackageLeader"
    Other = "Other"


class DateType(StrEnum):
    Accepted = "Accepted"
    Available = "Available"
    Copyrighted = "Copyrighted"
    Collected = "Collected"
    Coverage = "Coverage"
    Created = "Created"
    Issued = "Issued"
    Submitted = "Submitted"
    Updated = "Updated"
    Valid = "Valid"
    Withdrawn = "Withdrawn"
    Other = "Other"


class ContentTypeGeneral(StrEnum):
    Audiovisual = "Audiovisual"
    Award = "Award"
    Book = "Book"
    BookChapter = "BookChapter"
    Collection = "Collection"
    ComputationalNotebook = "ComputationalNotebook"
    ConferencePaper = "ConferencePaper"
    ConferenceProceeding = "ConferenceProceeding"
    DataPaper = "DataPaper"
    Dataset = "Dataset"
    Dissertation = "Dissertation"
    Event = "Event"
    Image = "Image"
    Instrument = "Instrument"
    InteractiveResource = "InteractiveResource"
    Journal = "Journal"
    JournalArticle = "JournalArticle"
    Model = "Model"
    OutputManagementPlan = "OutputManagementPlan"
    PeerReview = "PeerReview"
    PhysicalObject = "PhysicalObject"
    Preprint = "Preprint"
    Project = "Project"
    Report = "Report"
    Service = "Service"
    Software = "Software"
    Sound = "Sound"
    Standard = "Standard"
    StudyRegistration = "StudyRegistration"
    Text = "Text"
    Workflow = "Workflow"
    Other = "Other"


class DescriptionType(StrEnum):
    Abstract = "Abstract"
    Methods = "Methods"
    SeriesInformation = "SeriesInformation"
    TableOfContents = "TableOfContents"
    TechnicalInfo = "TechnicalInfo"
    Other = "Other"


class RelationType(StrEnum):
    IsCitedBy = "IsCitedBy"
    Cites = "Cites"
    IsCollectedBy = "IsCollectedBy"
    Collects = "Collects"
    IsSupplementTo = "IsSupplementTo"
    IsSupplementedBy = "IsSupplementedBy"
    IsContinuedBy = "IsContinuedBy"
    Continues = "Continues"
    IsDescribedBy = "IsDescribedBy"
    Describes = "Describes"
    HasMetadata = "HasMetadata"
    IsMetadataFor = "IsMetadataFor"
    HasVersion = "HasVersion"
    IsVersionOf = "IsVersionOf"
    IsNewVersionOf = "IsNewVersionOf"
    IsPartOf = "IsPartOf"
    IsPreviousVersionOf = "IsPreviousVersionOf"
    IsPublishedIn = "IsPublishedIn"
    HasPart = "HasPart"
    IsReferencedBy = "IsReferencedBy"
    References = "References"
    IsDocumentedBy = "IsDocumentedBy"
    Documents = "Documents"
    IsCompiledBy = "IsCompiledBy"
    Compiles = "Compiles"
    IsVariantFormOf = "IsVariantFormOf"
    IsOriginalFormOf = "IsOriginalFormOf"
    IsIdenticalTo = "IsIdenticalTo"
    IsReviewedBy = "IsReviewedBy"
    Reviews = "Reviews"
    IsDerivedFrom = "IsDerivedFrom"
    IsSourceOf = "IsSourceOf"
    IsRequiredBy = "IsRequiredBy"
    Requires = "Requires"
    IsObsoletedBy = "IsObsoletedBy"
    Obsoletes = "Obsoletes"
    HasTranslation = "HasTranslation"
    IsTranslationOf = "IsTranslationOf"


class RelatedIdentifierType(StrEnum):
    ARK = "ARK"
    arXiv = "arXiv"
    bibcode = "bibcode"
    CSTR = "CSTR"
    DOI = "DOI"
    EAN13 = "EAN13"
    EISSN = "EISSN"
    Handle = "Handle"
    IGSN = "IGSN"
    ISBN = "ISBN"
    ISSN = "ISSN"
    ISTC = "ISTC"
    LISSN = "LISSN"
    LSID = "LSID"
    PMID = "PMID"
    PURL = "PURL"
    RRID = "RRID"
    UPC = "UPC"
    URL = "URL"
    URN = "URN"
    w3id = "w3id"


class FunderIdentifierType(StrEnum):
    ISNI = "ISNI"
    GRID = "GRID"
    CrossrefFunderID = "Crossref Funder ID"
    ROR = "ROR"
    Other = "Other"


class NumberType(StrEnum):
    Article = "Article"
    Chapter = "Chapter"
    Report = "Report"
    Other = "Other"


Longitude = Annotated[float, Field(ge=-180, le=180)]

Latitude = Annotated[float, Field(ge=-90, le=90)]
