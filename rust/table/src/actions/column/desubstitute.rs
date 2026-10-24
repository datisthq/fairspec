use fairspec_metadata::{Column, get_base_property_type};
use polars::prelude::*;
use serde_json::Value;

use super::_shared::missing::get_missing_values;
use crate::models::column::{ColumnMapping, DenormalizeColumnOptions};

pub fn desubstitute_column(
    mapping: &ColumnMapping,
    column_expr: Expr,
    options: &DenormalizeColumnOptions,
) -> Expr {
    let Some(missing_value_type) = get_missing_value_type(&mapping.target, options) else {
        return column_expr;
    };

    let missing_value = get_missing_values(&mapping.target)
        .into_iter()
        .find(|value| match missing_value_type {
            "string" => value.is_string(),
            _ => value.is_number(),
        });
    let missing_value = match missing_value {
        Some(Value::String(text)) => lit(text),
        Some(Value::Number(number)) => match number.as_i64() {
            Some(integer) => lit(integer),
            None => lit(number.as_f64().unwrap_or_default()),
        },
        _ => return column_expr,
    };

    when(column_expr.clone().is_null())
        .then(missing_value)
        .otherwise(column_expr)
        .alias(mapping.target.name())
}

fn get_missing_value_type(
    column: &Column,
    options: &DenormalizeColumnOptions,
) -> Option<&'static str> {
    let property = serde_json::to_value(column.property()).unwrap_or_default();
    match get_base_property_type(property.get("type")) {
        Some("string") => Some("string"),
        Some(base_type @ ("integer" | "number")) => {
            let is_native = options
                .native_types
                .as_ref()
                .is_some_and(|types| types.iter().any(|item| item == base_type));
            Some(if is_native { "number" } else { "string" })
        }
        _ => None,
    }
}
