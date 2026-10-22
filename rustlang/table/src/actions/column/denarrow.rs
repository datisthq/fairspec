use fairspec_metadata::{CategoricalColumnProperty, Column};
use polars::prelude::*;

use super::_shared::missing::create_series;
use super::narrow::get_is_alphanumeric_variant;
use crate::helpers::column::get_categorical_values_and_labels;
use crate::models::column::ColumnMapping;

pub fn denarrow_column(mapping: &ColumnMapping, column_expr: Expr) -> Expr {
    if let Column::Categorical(column) = &mapping.target
        && get_is_alphanumeric_variant(&mapping.source.r#type)
    {
        let (values, labels) = get_categorical_values_and_labels(column);
        let polars_type = match column.property {
            CategoricalColumnProperty::String(_) => DataType::String,
            CategoricalColumnProperty::Integer(_) => DataType::Int64,
        };
        if !values.is_empty() {
            return column_expr.replace_strict(
                lit(Series::new("".into(), labels)),
                lit(create_series(&values)),
                Some(lit(NULL)),
                Some(polars_type),
            );
        }
    }
    column_expr
}
