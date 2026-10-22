use fairspec_metadata::Column;
use polars::prelude::*;

use super::types::base64::parse_base64_column;
use super::types::boolean::parse_boolean_column;
use super::types::date::parse_date_column;
use super::types::date_time::parse_date_time_column;
use super::types::decimal::parse_decimal_column;
use super::types::email::parse_email_column;
use super::types::hex::parse_hex_column;
use super::types::integer::parse_integer_column;
use super::types::list::parse_list_column;
use super::types::local_date_time::parse_local_date_time_column;
use super::types::local_time::parse_local_time_column;
use super::types::number::parse_number_column;
use super::types::time::parse_time_column;
use super::types::url::parse_url_column;
use crate::models::column::ColumnMapping;

pub fn parse_column(mapping: &ColumnMapping, column_expr: Expr) -> Expr {
    if mapping.source.r#type != DataType::String {
        return column_expr;
    }
    match &mapping.target {
        Column::Base64(column) => parse_base64_column(column, column_expr),
        Column::Boolean(column) => parse_boolean_column(column, column_expr),
        Column::Date(column) => parse_date_column(column, column_expr),
        Column::DateTime(column) => parse_date_time_column(column, column_expr),
        Column::Decimal(column) => parse_decimal_column(column, column_expr),
        Column::Email(column) => parse_email_column(column, column_expr),
        Column::Hex(column) => parse_hex_column(column, column_expr),
        Column::Integer(column) => parse_integer_column(column, column_expr),
        Column::List(column) => parse_list_column(column, column_expr),
        Column::LocalDateTime(column) => parse_local_date_time_column(column, column_expr),
        Column::LocalTime(column) => parse_local_time_column(column, column_expr),
        Column::Number(column) => parse_number_column(column, column_expr),
        Column::Time(column) => parse_time_column(column, column_expr),
        Column::Url(column) => parse_url_column(column, column_expr),
        _ => column_expr,
    }
}
