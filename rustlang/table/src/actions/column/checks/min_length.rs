use fairspec_metadata::{CellError, CellMinLengthError, Column};
use polars::prelude::*;

use super::check::ColumnCheck;
use crate::models::cell::CellMapping;

pub fn check_cell_min_length(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    let property = serde_json::to_value(column.property()).ok()?;
    let min_length = property
        .get("minLength")?
        .as_i64()
        .filter(|min_length| *min_length != 0)?;

    let is_error_expr = mapping.source.clone().str().len_chars().lt(lit(min_length));

    let error_template = CellError::MinLength(CellMinLengthError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
        min_length,
    });

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}
