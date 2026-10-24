use std::sync::LazyLock;

use fairspec_metadata::{CellError, Column, Result, TopojsonColumn};
use serde_json::Value;

use crate::actions::column::_shared::helpers::inspect_json_column;
use crate::models::column::ColumnMapping;
use crate::models::table::Table;
use crate::schemas::TOPOJSON_SCHEMA;

static TOPOJSON_JSON_SCHEMA: LazyLock<Value> =
    LazyLock::new(|| serde_json::from_str(TOPOJSON_SCHEMA).unwrap_or_default());

pub fn inspect_topojson_column(
    column: &TopojsonColumn,
    table: Table,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    inspect_json_column(
        &Column::Topojson(column.clone()),
        table,
        Some(&TOPOJSON_JSON_SCHEMA),
        mapping,
    )
}
