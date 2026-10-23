use fairspec_library::fairspec_metadata::{Error, Resource, Result};
use fairspec_library::fairspec_table::query_table;
use fairspec_library::load_table;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::helpers::limit::MAX_LIMIT;
use crate::tools::_shared::records::get_limited_records;
use crate::tools::_shared::schemas::check_range;
use crate::tools::dataset::query::get_default_limit;

#[derive(Debug, Clone, Deserialize)]
pub struct QueryTableInput {
    pub resource: Resource,
    pub query: String,
    #[serde(default = "get_default_limit")]
    pub limit: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct QueryTableResult {
    pub records: Vec<Map<String, Value>>,
    pub truncated: bool,
}

/// Query a table using SQL, exposed as `self`.
pub fn query_table_tool(input: QueryTableInput) -> Result<QueryTableResult> {
    check_range("limit", Some(input.limit), 1, MAX_LIMIT)?;
    let Some(table) = load_table(&input.resource, &Default::default())? else {
        return Err(Error::Load("Could not load table".to_string()));
    };
    let (records, truncated) = get_limited_records(query_table(table, &input.query)?, input.limit)?;
    Ok(QueryTableResult { records, truncated })
}
