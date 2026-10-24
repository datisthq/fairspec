use fairspec_metadata::{CellError, CellMinItemsError, Column, ColumnType};
use polars::prelude::*;

use super::check::ColumnCheck;
use crate::models::cell::CellMapping;

pub fn check_cell_min_items(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    if column.column_type() != ColumnType::List {
        return None;
    }

    let property = serde_json::to_value(column.property()).ok()?;
    let min_items = property.get("minItems")?.as_i64()?;

    let is_error_expr = mapping.target.clone().list().len().lt(lit(min_items));

    let error_template = CellError::MinItems(CellMinItemsError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
        min_items,
    });

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}
