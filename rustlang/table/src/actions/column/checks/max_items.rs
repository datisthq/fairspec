use fairspec_metadata::{CellError, CellMaxItemsError, Column, ColumnType};
use polars::prelude::*;

use super::check::ColumnCheck;
use crate::models::cell::CellMapping;

pub fn check_cell_max_items(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    if column.column_type() != ColumnType::List {
        return None;
    }

    let property = serde_json::to_value(column.property()).ok()?;
    let max_items = property.get("maxItems")?.as_i64()?;

    let is_error_expr = mapping.target.clone().list().len().gt(lit(max_items));

    let error_template = CellError::MaxItems(CellMaxItemsError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
        max_items,
    });

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}
