use fairspec_metadata::Column;
use polars::prelude::DataType;
use serde::{Deserialize, Serialize};

/// A source column as polars sees it; `time_zone` is set for a zoned datetime.
#[derive(Debug, Clone, PartialEq)]
pub struct PolarsColumn {
    pub name: String,
    pub r#type: DataType,
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnMapping {
    pub source: PolarsColumn,
    pub target: Column,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DenormalizeColumnOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_types: Option<Vec<String>>,
}
