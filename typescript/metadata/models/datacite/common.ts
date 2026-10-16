import { z } from "zod"

export const CreatorNameType = z.enum(["Organizational", "Personal"])

export const TitleType = z.enum([
  "AlternativeTitle",
  "Subtitle",
  "TranslatedTitle",
  "Other",
])

export const ContributorType = z.enum([
  "ContactPerson",
  "DataCollector",
  "DataCurator",
  "DataManager",
  "Distributor",
  "Editor",
  "HostingInstitution",
  "Producer",
  "ProjectLeader",
  "ProjectManager",
  "ProjectMember",
  "RegistrationAgency",
  "RegistrationAuthority",
  "RelatedPerson",
  "Researcher",
  "ResearchGroup",
  "RightsHolder",
  "Sponsor",
  "Supervisor",
  "Translator",
  "WorkPackageLeader",
  "Other",
])

export const DateType = z.enum([
  "Accepted",
  "Available",
  "Copyrighted",
  "Collected",
  "Coverage",
  "Created",
  "Issued",
  "Submitted",
  "Updated",
  "Valid",
  "Withdrawn",
  "Other",
])

export const ContentTypeGeneral = z.enum([
  "Audiovisual",
  "Award",
  "Book",
  "BookChapter",
  "Collection",
  "ComputationalNotebook",
  "ConferencePaper",
  "ConferenceProceeding",
  "DataPaper",
  "Dataset",
  "Dissertation",
  "Event",
  "Image",
  "Instrument",
  "InteractiveResource",
  "Journal",
  "JournalArticle",
  "Model",
  "OutputManagementPlan",
  "PeerReview",
  "PhysicalObject",
  "Preprint",
  "Project",
  "Report",
  "Service",
  "Software",
  "Sound",
  "Standard",
  "StudyRegistration",
  "Text",
  "Workflow",
  "Other",
])

export const DescriptionType = z.enum([
  "Abstract",
  "Methods",
  "SeriesInformation",
  "TableOfContents",
  "TechnicalInfo",
  "Other",
])

export const RelationType = z.enum([
  "IsCitedBy",
  "Cites",
  "IsCollectedBy",
  "Collects",
  "IsSupplementTo",
  "IsSupplementedBy",
  "IsContinuedBy",
  "Continues",
  "IsDescribedBy",
  "Describes",
  "HasMetadata",
  "IsMetadataFor",
  "HasVersion",
  "IsVersionOf",
  "IsNewVersionOf",
  "IsPartOf",
  "IsPreviousVersionOf",
  "IsPublishedIn",
  "HasPart",
  "IsReferencedBy",
  "References",
  "IsDocumentedBy",
  "Documents",
  "IsCompiledBy",
  "Compiles",
  "IsVariantFormOf",
  "IsOriginalFormOf",
  "IsIdenticalTo",
  "IsReviewedBy",
  "Reviews",
  "IsDerivedFrom",
  "IsSourceOf",
  "IsRequiredBy",
  "Requires",
  "IsObsoletedBy",
  "Obsoletes",
  "HasTranslation",
  "IsTranslationOf",
])

export const RelatedIdentifierType = z.enum([
  "ARK",
  "arXiv",
  "bibcode",
  "CSTR",
  "DOI",
  "EAN13",
  "EISSN",
  "Handle",
  "IGSN",
  "ISBN",
  "ISSN",
  "ISTC",
  "LISSN",
  "LSID",
  "PMID",
  "PURL",
  "RRID",
  "UPC",
  "URL",
  "URN",
  "w3id",
])

export const FunderIdentifierType = z.enum([
  "ISNI",
  "GRID",
  "Crossref Funder ID",
  "ROR",
  "Other",
])

export const NumberType = z.enum(["Article", "Chapter", "Report", "Other"])

export const Longitude = z.number().min(-180).max(180)

export const Latitude = z.number().min(-90).max(90)

export type CreatorNameType = z.infer<typeof CreatorNameType>
export type TitleType = z.infer<typeof TitleType>
export type ContributorType = z.infer<typeof ContributorType>
export type DateType = z.infer<typeof DateType>
export type ContentTypeGeneral = z.infer<typeof ContentTypeGeneral>
export type DescriptionType = z.infer<typeof DescriptionType>
export type RelationType = z.infer<typeof RelationType>
export type RelatedIdentifierType = z.infer<typeof RelatedIdentifierType>
export type FunderIdentifierType = z.infer<typeof FunderIdentifierType>
export type NumberType = z.infer<typeof NumberType>
export type Longitude = z.infer<typeof Longitude>
export type Latitude = z.infer<typeof Latitude>
