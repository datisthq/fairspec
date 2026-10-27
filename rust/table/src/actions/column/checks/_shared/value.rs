use fairspec_metadata::{CategoricalColumnProperty, Column};
use polars::prelude::*;

use crate::actions::column::types::integer::{normalize_integer_text, parse_integer_text};
use crate::models::cell::CellMapping;

/// The cell value a numeric constraint compares, or None for a column it ignores
pub fn get_numeric_value(column: &Column, mapping: &CellMapping) -> Option<Expr> {
    match column {
        Column::Integer(_) | Column::Number(_) | Column::Decimal(_) => Some(mapping.target.clone()),
        // The target of a categorical column is its label, so the value is read again
        Column::Categorical(column) => match &column.property {
            CategoricalColumnProperty::Integer(property) => {
                let text_expr = mapping.source.clone().cast(DataType::String);
                Some(parse_integer_text(normalize_integer_text(
                    &property.base,
                    text_expr,
                )))
            }
            CategoricalColumnProperty::String(_) => None,
        },
        _ => None,
    }
}
