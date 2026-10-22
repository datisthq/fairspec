use fairspec_metadata::{CellError, CellExclusiveMaximumError, CellMaximumError, Column};
use polars::prelude::*;
use serde_json::Value;

use super::check::ColumnCheck;
use crate::helpers::general::stringify_number;
use crate::models::cell::CellMapping;

pub fn check_cell_maximum(
    column: &Column,
    mapping: &CellMapping,
    is_exclusive: bool,
) -> Option<ColumnCheck> {
    let property = serde_json::to_value(column.property()).ok()?;
    let key = if is_exclusive {
        "exclusiveMaximum"
    } else {
        "maximum"
    };
    let Some(Value::Number(maximum)) = property.get(key) else {
        return None;
    };

    let (literal, text) = match maximum.as_i64() {
        Some(integer) => (lit(integer), integer.to_string()),
        None => {
            let number = maximum.as_f64().unwrap_or_default();
            (lit(number), stringify_number(number))
        }
    };

    let target = mapping.target.clone();
    let is_error_expr = if is_exclusive {
        target.gt_eq(literal)
    } else {
        target.gt(literal)
    };

    let column_name = column.name().to_string();
    let error_template = if is_exclusive {
        CellError::ExclusiveMaximum(CellExclusiveMaximumError {
            resource_name: None,
            column_name,
            row_number: 0,
            cell: String::new(),
            maximum: text,
        })
    } else {
        CellError::Maximum(CellMaximumError {
            resource_name: None,
            column_name,
            row_number: 0,
            cell: String::new(),
            maximum: text,
        })
    };

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}
