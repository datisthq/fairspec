use fairspec_metadata::Column;
use polars::prelude::*;

use super::_shared::missing::create_series;
use crate::helpers::column::get_categorical_values_and_labels;
use crate::models::column::ColumnMapping;

pub fn narrow_column(mapping: &ColumnMapping, column_expr: Expr) -> Expr {
    let variant = &mapping.source.r#type;
    let mut column_expr = column_expr;

    if matches!(mapping.target, Column::Boolean(_)) && get_is_integer_variant(variant) {
        column_expr = when(column_expr.clone().eq(lit(1)))
            .then(lit(true))
            .when(column_expr.eq(lit(0)))
            .then(lit(false))
            .otherwise(lit(NULL));
    }

    if matches!(mapping.target, Column::Integer(_)) && get_is_number_variant(variant) {
        column_expr = when(
            column_expr
                .clone()
                .eq(column_expr.clone().round(0, RoundMode::HalfToEven)),
        )
        .then(column_expr.cast(DataType::Int64))
        .otherwise(lit(NULL));
    }

    if let Column::Categorical(column) = &mapping.target
        && get_is_alphanumeric_variant(variant)
    {
        let (values, labels) = get_categorical_values_and_labels(column);
        if !values.is_empty() {
            return column_expr.replace_strict(
                lit(create_series(&values)),
                lit(Series::new("".into(), labels)),
                Some(lit(NULL)),
                Some(DataType::from_categories(Categories::global())),
            );
        }
    }

    column_expr
}

pub(crate) fn get_is_integer_variant(variant: &DataType) -> bool {
    matches!(
        variant,
        DataType::Int8 | DataType::Int16 | DataType::Int32 | DataType::Int64
    )
}

pub(crate) fn get_is_number_variant(variant: &DataType) -> bool {
    matches!(variant, DataType::Float32 | DataType::Float64)
}

pub(crate) fn get_is_alphanumeric_variant(variant: &DataType) -> bool {
    get_is_integer_variant(variant)
        || get_is_number_variant(variant)
        || matches!(variant, DataType::String | DataType::Categorical(_, _))
}
