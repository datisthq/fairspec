use polars::prelude::*;

use super::denarrow::denarrow_column;
use super::desubstitute::desubstitute_column;
use super::stringify::stringify_column;
use crate::models::column::{ColumnMapping, DenormalizeColumnOptions};

pub fn denormalize_column(mapping: &ColumnMapping, options: &DenormalizeColumnOptions) -> Expr {
    let mut column_expr = col(mapping.source.name.as_str());

    let column_type = serde_json::to_value(mapping.target.column_type()).unwrap_or_default();
    let is_native = options.native_types.as_ref().is_some_and(|types| {
        types
            .iter()
            .any(|item| Some(item.as_str()) == column_type.as_str())
    });
    if !is_native {
        column_expr = denarrow_column(mapping, column_expr);
        column_expr = stringify_column(mapping, column_expr);
    }

    desubstitute_column(mapping, column_expr, options)
}
