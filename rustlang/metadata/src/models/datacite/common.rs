use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreatorNameType {
    Organizational,
    Personal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TitleType {
    AlternativeTitle,
    Subtitle,
    TranslatedTitle,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContributorType {
    ContactPerson,
    DataCollector,
    DataCurator,
    DataManager,
    Distributor,
    Editor,
    HostingInstitution,
    Producer,
    ProjectLeader,
    ProjectManager,
    ProjectMember,
    RegistrationAgency,
    RegistrationAuthority,
    RelatedPerson,
    Researcher,
    ResearchGroup,
    RightsHolder,
    Sponsor,
    Supervisor,
    Translator,
    WorkPackageLeader,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DateType {
    Accepted,
    Available,
    Copyrighted,
    Collected,
    Coverage,
    Created,
    Issued,
    Submitted,
    Updated,
    Valid,
    Withdrawn,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentTypeGeneral {
    Audiovisual,
    Award,
    Book,
    BookChapter,
    Collection,
    ComputationalNotebook,
    ConferencePaper,
    ConferenceProceeding,
    DataPaper,
    Dataset,
    Dissertation,
    Event,
    Image,
    Instrument,
    InteractiveResource,
    Journal,
    JournalArticle,
    Model,
    OutputManagementPlan,
    PeerReview,
    PhysicalObject,
    Preprint,
    Project,
    Report,
    Service,
    Software,
    Sound,
    Standard,
    StudyRegistration,
    Text,
    Workflow,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DescriptionType {
    Abstract,
    Methods,
    SeriesInformation,
    TableOfContents,
    TechnicalInfo,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationType {
    IsCitedBy,
    Cites,
    IsCollectedBy,
    Collects,
    IsSupplementTo,
    IsSupplementedBy,
    IsContinuedBy,
    Continues,
    IsDescribedBy,
    Describes,
    HasMetadata,
    IsMetadataFor,
    HasVersion,
    IsVersionOf,
    IsNewVersionOf,
    IsPartOf,
    IsPreviousVersionOf,
    IsPublishedIn,
    HasPart,
    IsReferencedBy,
    References,
    IsDocumentedBy,
    Documents,
    IsCompiledBy,
    Compiles,
    IsVariantFormOf,
    IsOriginalFormOf,
    IsIdenticalTo,
    IsReviewedBy,
    Reviews,
    IsDerivedFrom,
    IsSourceOf,
    IsRequiredBy,
    Requires,
    IsObsoletedBy,
    Obsoletes,
    HasTranslation,
    IsTranslationOf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelatedIdentifierType {
    #[serde(rename = "ARK")]
    Ark,
    #[serde(rename = "arXiv")]
    ArXiv,
    #[serde(rename = "bibcode")]
    Bibcode,
    #[serde(rename = "CSTR")]
    Cstr,
    #[serde(rename = "DOI")]
    Doi,
    #[serde(rename = "EAN13")]
    Ean13,
    #[serde(rename = "EISSN")]
    Eissn,
    Handle,
    #[serde(rename = "IGSN")]
    Igsn,
    #[serde(rename = "ISBN")]
    Isbn,
    #[serde(rename = "ISSN")]
    Issn,
    #[serde(rename = "ISTC")]
    Istc,
    #[serde(rename = "LISSN")]
    Lissn,
    #[serde(rename = "LSID")]
    Lsid,
    #[serde(rename = "PMID")]
    Pmid,
    #[serde(rename = "PURL")]
    Purl,
    #[serde(rename = "RRID")]
    Rrid,
    #[serde(rename = "UPC")]
    Upc,
    #[serde(rename = "URL")]
    Url,
    #[serde(rename = "URN")]
    Urn,
    #[serde(rename = "w3id")]
    W3id,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FunderIdentifierType {
    #[serde(rename = "ISNI")]
    Isni,
    #[serde(rename = "GRID")]
    Grid,
    #[serde(rename = "Crossref Funder ID")]
    CrossrefFunderID,
    #[serde(rename = "ROR")]
    Ror,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NumberType {
    Article,
    Chapter,
    Report,
    Other,
}

pub type Longitude = f64;
pub type Latitude = f64;
