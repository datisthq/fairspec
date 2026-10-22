use fairspec_metadata::BooleanColumn;
use polars::prelude::*;

const DEFAULT_TRUE_VALUES: [&str; 4] = ["true", "True", "TRUE", "1"];
const DEFAULT_FALSE_VALUES: [&str; 4] = ["false", "False", "FALSE", "0"];

const DEFAULT_TRUE_VALUE: &str = "true";
const DEFAULT_FALSE_VALUE: &str = "false";

pub fn parse_boolean_column(column: &BooleanColumn, column_expr: Expr) -> Expr {
    let true_values = get_values(&column.property.true_values, &DEFAULT_TRUE_VALUES);
    let false_values = get_values(&column.property.false_values, &DEFAULT_FALSE_VALUES);

    // The values go into the pattern unescaped, as in Python
    let mut column_expr = column_expr;
    for value in true_values {
        column_expr = column_expr
            .str()
            .replace(lit(format!("^{value}$")), lit("1"), false);
    }
    for value in false_values {
        column_expr = column_expr
            .str()
            .replace(lit(format!("^{value}$")), lit("0"), false);
    }

    let column_expr = column_expr.cast(DataType::Int8);
    when(column_expr.clone().eq(lit(1)))
        .then(lit(true))
        .when(column_expr.eq(lit(0)))
        .then(lit(false))
        .otherwise(lit(NULL))
        .alias(column.name.as_str())
}

pub fn stringify_boolean_column(column: &BooleanColumn, column_expr: Expr) -> Expr {
    let true_value = column
        .property
        .true_values
        .as_ref()
        .and_then(|values| values.first())
        .map_or(DEFAULT_TRUE_VALUE, String::as_str);
    let false_value = column
        .property
        .false_values
        .as_ref()
        .and_then(|values| values.first())
        .map_or(DEFAULT_FALSE_VALUE, String::as_str);

    when(column_expr.eq(lit(true)))
        .then(lit(true_value))
        .otherwise(lit(false_value))
        .alias(column.name.as_str())
}

fn get_values(values: &Option<Vec<String>>, defaults: &[&str]) -> Vec<String> {
    match values {
        Some(values) if !values.is_empty() => values.clone(),
        _ => defaults.iter().map(|value| value.to_string()).collect(),
    }
}
