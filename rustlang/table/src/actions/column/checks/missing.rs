use fairspec_metadata::{CellError, CellMissingError, Column, ColumnType};

use super::check::ColumnCheck;
use crate::models::cell::CellMapping;

pub fn check_cell_missing(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    if column.nullable().unwrap_or(false) || column.column_type() == ColumnType::Unknown {
        return None;
    }

    let is_error_expr = mapping.target.clone().is_null();

    let error_template = CellError::Missing(CellMissingError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
    });

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}
