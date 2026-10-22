use fairspec_metadata::{CellError, CellTypeError, Column};

use super::check::ColumnCheck;
use crate::models::cell::CellMapping;

pub fn check_cell_type(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    let is_error_expr = mapping
        .source
        .clone()
        .is_not_null()
        .and(mapping.target.clone().is_null());

    let error_template = CellError::Type(CellTypeError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
        column_type: column.column_type(),
    });

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}
