use fairspec_metadata::{CellError, CellExclusiveMinimumError, CellMinimumError, Column};
use polars::prelude::*;
use serde_json::Value;

use super::_shared::value::get_numeric_value;
use super::check::ColumnCheck;
use crate::helpers::general::stringify_number;
use crate::models::cell::CellMapping;

pub fn check_cell_minimum(
    column: &Column,
    mapping: &CellMapping,
    is_exclusive: bool,
) -> Option<ColumnCheck> {
    let target = get_numeric_value(column, mapping)?;
    let property = serde_json::to_value(column.property()).ok()?;
    let key = if is_exclusive {
        "exclusiveMinimum"
    } else {
        "minimum"
    };
    let Some(Value::Number(minimum)) = property.get(key) else {
        return None;
    };

    let (literal, text) = match minimum.as_i64() {
        Some(integer) => (lit(integer), integer.to_string()),
        None => {
            let number = minimum.as_f64().unwrap_or_default();
            (lit(number), stringify_number(number))
        }
    };

    let is_error_expr = if is_exclusive {
        target.lt_eq(literal)
    } else {
        target.lt(literal)
    };

    let column_name = column.name().to_string();
    let error_template = if is_exclusive {
        CellError::ExclusiveMinimum(CellExclusiveMinimumError {
            resource_name: None,
            column_name,
            row_number: 0,
            cell: String::new(),
            minimum: text,
        })
    } else {
        CellError::Minimum(CellMinimumError {
            resource_name: None,
            column_name,
            row_number: 0,
            cell: String::new(),
            minimum: text,
        })
    };

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}
