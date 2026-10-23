use fairspec_library::fairspec_metadata::Result;
use fairspec_library::query_dataset;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::helpers::limit::{DEFAULT_LIMIT, MAX_LIMIT};
use crate::tools::_shared::records::get_limited_records;
use crate::tools::_shared::schemas::check_range;
use crate::tools::_shared::source::DatasetInput;

#[derive(Debug, Clone, Deserialize)]
pub struct QueryDatasetInput {
    pub source: DatasetInput,
    pub query: String,
    #[serde(default = "get_default_limit")]
    pub limit: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryDatasetResult {
    pub records: Vec<Map<String, Value>>,
    pub truncated: bool,
}

/// Query a dataset using SQL, each table resource under its name.
pub fn query_dataset_tool(input: QueryDatasetInput) -> Result<QueryDatasetResult> {
    check_range("limit", Some(input.limit), 1, MAX_LIMIT)?;
    let table = query_dataset(input.source, &input.query, &Default::default())?;
    let (records, truncated) = get_limited_records(table, input.limit)?;
    Ok(QueryDatasetResult { records, truncated })
}

pub(crate) fn get_default_limit() -> usize {
    DEFAULT_LIMIT
}
