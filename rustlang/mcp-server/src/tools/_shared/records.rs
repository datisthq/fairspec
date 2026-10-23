use fairspec_library::fairspec_metadata::{Error, Result};
use fairspec_library::fairspec_table::Table;
use fairspec_library::fairspec_table::polars::prelude::*;
use serde_json::{Map, Value};

use crate::helpers::limit::limit_items;

/// The first `limit` rows as JSON records, and whether more were left out.
pub fn get_limited_records(table: Table, limit: usize) -> Result<(Vec<Map<String, Value>>, bool)> {
    let to_error = |error: PolarsError| Error::Load(error.to_string());
    let mut frame = table
        .limit((limit + 1) as IdxSize)
        .collect()
        .map_err(to_error)?;
    let mut text = Vec::new();
    JsonWriter::new(&mut text)
        .with_json_format(JsonFormat::Json)
        .finish(&mut frame)
        .map_err(to_error)?;
    let records = serde_json::from_slice(&text).map_err(|error| Error::Parse(error.to_string()))?;
    Ok(limit_items(records, limit))
}
