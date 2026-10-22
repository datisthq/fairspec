use std::sync::LazyLock;

use fairspec_metadata::{CellError, Column, GeojsonColumn, Result};
use serde_json::Value;

use crate::actions::column::_shared::helpers::inspect_json_column;
use crate::models::column::ColumnMapping;
use crate::models::table::Table;
use crate::schemas::GEOJSON_SCHEMA;

static GEOJSON_JSON_SCHEMA: LazyLock<Value> =
    LazyLock::new(|| serde_json::from_str(GEOJSON_SCHEMA).unwrap_or_default());

pub fn inspect_geojson_column(
    column: &GeojsonColumn,
    table: Table,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    inspect_json_column(
        &Column::Geojson(column.clone()),
        table,
        Some(&GEOJSON_JSON_SCHEMA),
        mapping,
    )
}
