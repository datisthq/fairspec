use serde::{Deserialize, Serialize};

use super::data::DataError;
use super::file::FileError;
use super::metadata::MetadataError;
use super::resource::ResourceError;
use super::table::TableError;

/// Every error variant is selected by its `type` string.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FairspecError {
    Metadata(MetadataError),
    Resource(ResourceError),
    Table(TableError),
    Data(DataError),
    File(FileError),
}
