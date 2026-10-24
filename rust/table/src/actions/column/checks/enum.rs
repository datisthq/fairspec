use fairspec_metadata::{CellEnumError, CellError, Column};
use polars::prelude::*;
use serde_json::Value;

use super::check::ColumnCheck;
use crate::actions::column::_shared::python::{dump_json, stringify_value};
use crate::models::cell::CellMapping;

pub fn check_cell_enum(column: &Column, mapping: &CellMapping) -> Option<ColumnCheck> {
    let property = serde_json::to_value(column.property()).ok()?;
    let Some(Value::Array(enum_constraint)) = property.get("enum") else {
        return None;
    };

    let primitive_enum_constraint: Vec<Value> = enum_constraint
        .iter()
        .map(|item| match item {
            Value::Object(_) | Value::Array(_) => Value::String(dump_json(item)),
            other => other.clone(),
        })
        .collect();

    let is_error_expr = mapping
        .target
        .clone()
        .is_in(
            lit(create_series(&primitive_enum_constraint)).implode(true),
            false,
        )
        .not();

    let error_template = CellError::Enum(CellEnumError {
        resource_name: None,
        column_name: column.name().to_string(),
        row_number: 0,
        cell: String::new(),
        r#enum: enum_constraint.iter().map(stringify_value).collect(),
    });

    Some(ColumnCheck {
        is_error_expr,
        error_template,
    })
}

fn create_series(values: &[Value]) -> Series {
    let items = || values.iter().filter(|value| !value.is_null());
    if items().all(Value::is_boolean) {
        let booleans: Vec<Option<bool>> = values.iter().map(Value::as_bool).collect();
        return Series::new("".into(), booleans);
    }
    if items().all(|value| value.as_i64().is_some()) {
        let integers: Vec<Option<i64>> = values.iter().map(Value::as_i64).collect();
        return Series::new("".into(), integers);
    }
    if items().all(Value::is_number) {
        let numbers: Vec<Option<f64>> = values.iter().map(Value::as_f64).collect();
        return Series::new("".into(), numbers);
    }
    let texts: Vec<Option<String>> = values
        .iter()
        .map(|value| (!value.is_null()).then(|| stringify_value(value)))
        .collect();
    Series::new("".into(), texts)
}
