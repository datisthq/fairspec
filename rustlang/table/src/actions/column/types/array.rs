use fairspec_metadata::{ArrayColumn, CellError, Column, Result};

use crate::actions::column::_shared::helpers::inspect_json_column;
use crate::models::column::ColumnMapping;
use crate::models::table::Table;

pub fn inspect_array_column(
    column: &ArrayColumn,
    table: Table,
    mapping: Option<&ColumnMapping>,
) -> Result<Vec<CellError>> {
    inspect_json_column(&Column::Array(column.clone()), table, None, mapping)
}
