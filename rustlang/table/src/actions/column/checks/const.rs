use fairspec_metadata::{CellConstError, CellError, Column};
use polars::prelude::*;
use serde_json::Value;

use super::check::ColumnCheck;
use crate::actions::column::_shared::python::{dump_json, stringify_value};
use crate::models::cell::CellMapping;

pub fn check_cell_const(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    let property = serde_json::to_value(column.property()).ok()?;
    let const_constraint = property.get("const").filter(|value| !value.is_null())?;

    let primitive_const_constraint = match const_constraint {
        Value::Object(_) | Value::Array(_) => Value::String(dump_json(const_constraint)),
        other => other.clone(),
    };

    let is_error_expr = mapping
        .target
        .clone()
        .eq(create_literal(&primitive_const_constraint))
        .not();

    let error_template = CellError::Const(CellConstError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
        r#const: stringify_value(&primitive_const_constraint),
    });

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}

fn create_literal(value: &Value) -> Expr {
    match value {
        Value::Bool(value) => lit(*value),
        Value::Number(number) => match number.as_i64() {
            Some(integer) => lit(integer),
            None => lit(number.as_f64().unwrap_or_default()),
        },
        Value::String(text) => lit(text.clone()),
        _ => lit(NULL),
    }
}
