use polars::prelude::*;
use serde_json::Value;

use super::_shared::missing::get_missing_values;
use crate::models::column::{ColumnMapping, PolarsColumn};

pub fn substitute_column(mapping: &ColumnMapping, column_expr: Expr) -> Expr {
    let Some(missing_value_type) = get_missing_value_type(&mapping.source) else {
        return column_expr;
    };
    let missing_values = get_missing_values(&mapping.target);

    let compatible: Series = if missing_value_type == "string" {
        let values: Vec<&str> = missing_values.iter().filter_map(Value::as_str).collect();
        Series::new("".into(), values)
    } else {
        let numbers: Vec<f64> = missing_values.iter().filter_map(Value::as_f64).collect();
        // is_in compares within one kind only: an integer column never equals 2.5
        if matches!(mapping.source.r#type, DataType::Float32 | DataType::Float64) {
            Series::new("".into(), numbers)
        } else {
            let integers: Vec<i64> = numbers
                .iter()
                .filter(|number| number.fract() == 0.0)
                .map(|number| *number as i64)
                .collect();
            Series::new("".into(), integers)
        }
    };
    if compatible.is_empty() {
        return column_expr;
    }

    when(
        column_expr
            .clone()
            .is_in(lit(compatible).implode(true), false),
    )
    .then(lit(NULL))
    .otherwise(column_expr)
    .alias(mapping.target.name())
}

fn get_missing_value_type(polars_column: &PolarsColumn) -> Option<&'static str> {
    match polars_column.r#type {
        DataType::String => Some("string"),
        DataType::Int8
        | DataType::Int16
        | DataType::Int32
        | DataType::Int64
        | DataType::Float32
        | DataType::Float64 => Some("number"),
        _ => None,
    }
}
