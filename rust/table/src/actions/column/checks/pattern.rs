use fairspec_metadata::{CellError, CellPatternError, Column};
use polars::prelude::*;

use super::check::ColumnCheck;
use crate::models::cell::CellMapping;

pub fn check_cell_pattern(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    let property = serde_json::to_value(column.property()).ok()?;
    let pattern = property
        .get("pattern")?
        .as_str()
        .filter(|pattern| !pattern.is_empty())?;

    let is_error_expr = mapping
        .source
        .clone()
        .str()
        .contains(lit(pattern), true)
        .not();

    let error_template = CellError::Pattern(CellPatternError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
        pattern: pattern.to_string(),
    });

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}
