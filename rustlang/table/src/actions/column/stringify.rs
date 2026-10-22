use fairspec_metadata::Column;
use polars::prelude::*;

use super::types::boolean::stringify_boolean_column;
use super::types::date::stringify_date_column;
use super::types::date_time::stringify_date_time_column;
use super::types::decimal::stringify_decimal_column;
use super::types::integer::stringify_integer_column;
use super::types::list::stringify_list_column;
use super::types::local_date_time::stringify_local_date_time_column;
use super::types::local_time::stringify_local_time_column;
use super::types::number::stringify_number_column;
use super::types::time::stringify_time_column;
use super::types::unknown::stringify_unknown_column;
use crate::models::column::ColumnMapping;

pub fn stringify_column(mapping: &ColumnMapping, column_expr: Expr) -> Expr {
    if mapping.source.r#type == DataType::String {
        return column_expr;
    }
    match &mapping.target {
        Column::Boolean(column) => stringify_boolean_column(column, column_expr),
        Column::Date(column) => stringify_date_column(column, column_expr),
        Column::DateTime(column) => {
            let column_expr = if mapping.source.time_zone.is_some() {
                column_expr.dt().convert_time_zone(TimeZone::UTC)
            } else {
                column_expr
            };
            stringify_date_time_column(column, column_expr)
        }
        Column::Decimal(column) => stringify_decimal_column(column, column_expr),
        Column::Integer(column) => stringify_integer_column(column, column_expr),
        Column::List(column) => stringify_list_column(column, column_expr),
        Column::LocalDateTime(column) => stringify_local_date_time_column(column, column_expr),
        Column::LocalTime(column) => stringify_local_time_column(column, column_expr),
        Column::Number(column) => stringify_number_column(column, column_expr),
        Column::Time(column) => stringify_time_column(column, column_expr),
        Column::Unknown(column) => stringify_unknown_column(column, column_expr),
        _ => column_expr,
    }
}
