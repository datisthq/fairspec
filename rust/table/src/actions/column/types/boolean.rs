use fairspec_metadata::BooleanColumn;
use polars::prelude::*;

const DEFAULT_TRUE_VALUES: [&str; 4] = ["true", "True", "TRUE", "1"];
const DEFAULT_FALSE_VALUES: [&str; 4] = ["false", "False", "FALSE", "0"];

const DEFAULT_TRUE_VALUE: &str = "true";
const DEFAULT_FALSE_VALUE: &str = "false";

pub fn parse_boolean_column(column: &BooleanColumn, column_expr: Expr) -> Expr {
    let true_values = get_values(&column.property.true_values, &DEFAULT_TRUE_VALUES);
    let false_values = get_values(&column.property.false_values, &DEFAULT_FALSE_VALUES);

    when(column_expr.clone().is_in(
        lit(Series::new("".into(), true_values)).implode(true),
        false,
    ))
    .then(lit(true))
    .when(column_expr.is_in(
        lit(Series::new("".into(), false_values)).implode(true),
        false,
    ))
    .then(lit(false))
    .otherwise(lit(NULL).cast(DataType::Boolean))
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
        Some(values) => values.clone(),
        None => defaults.iter().map(|value| value.to_string()).collect(),
    }
}
