use serde::{Deserialize, Serialize};

use super::alternate_identifier::AlternateIdentifiers;
use super::content_type::ContentTypes;
use super::contributor::Contributors;
use super::creator::Creators;
use super::date::Dates;
use super::description::Descriptions;
use super::formats::Formats;
use super::funding_reference::FundingReferences;
use super::geo_location::GeoLocations;
use super::identifier::{Doi, DoiPrefix, DoiSuffix};
use super::language::Language;
use super::publication_year::PublicationYear;
use super::publisher::Publisher;
use super::related_identifier::RelatedIdentifiers;
use super::related_item::RelatedItems;
use super::rights::RightsList;
use super::size::Sizes;
use super::subject::Subjects;
use super::title::Titles;
use super::version::Version;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datacite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doi: Option<Doi>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<DoiPrefix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suffix: Option<DoiSuffix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creators: Option<Creators>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub titles: Option<Titles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<Publisher>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publication_year: Option<PublicationYear>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Subjects>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Contributors>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<Dates>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<ContentTypes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate_identifiers: Option<AlternateIdentifiers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_identifiers: Option<RelatedIdentifiers>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Sizes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formats: Option<Formats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rights_list: Option<RightsList>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Descriptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geo_locations: Option<GeoLocations>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub funding_references: Option<FundingReferences>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_items: Option<RelatedItems>,
}
