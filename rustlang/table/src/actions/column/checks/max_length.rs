use fairspec_metadata::{CellError, CellMaxLengthError, Column};
use polars::prelude::*;

use super::check::ColumnCheck;
use crate::models::cell::CellMapping;

pub fn check_cell_max_length(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    let property = serde_json::to_value(column.property()).ok()?;
    let max_length = property
        .get("maxLength")?
        .as_i64()
        .filter(|max_length| *max_length != 0)?;

    let is_error_expr = mapping.source.clone().str().len_chars().gt(lit(max_length));

    let error_template = CellError::MaxLength(CellMaxLengthError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
        max_length,
    });

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}
