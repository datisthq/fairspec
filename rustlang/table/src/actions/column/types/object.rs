use fairspec_metadata::{CellError, Column, ObjectColumn, Result};

use crate::actions::column::_shared::helpers::inspect_json_column;
use crate::models::column::ColumnMapping;
use crate::models::table::Table;

pub fn inspect_object_column(
    column: &ObjectColumn,
    table: Table,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    inspect_json_column(&Column::Object(column.clone()), table, None, mapping)
}
