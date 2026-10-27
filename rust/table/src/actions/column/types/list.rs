use fairspec_metadata::{ListColumn, ListItemType};
use polars::prelude::*;

use super::date::parse_date_text;
use super::date_time::parse_date_time_text;
use super::integer::parse_integer_text;
use super::local_date_time::parse_local_date_time_text;
use super::local_time::parse_local_time_text;
use super::number::parse_number_text;
use super::time::parse_time_text;

const TRUE_VALUES: [&str; 4] = ["true", "True", "TRUE", "1"];
const FALSE_VALUES: [&str; 4] = ["false", "False", "FALSE", "0"];

pub fn parse_list_column(column: &ListColumn, column_expr: Expr) -> Expr {
    let delimiter = get_delimiter(column);

    // An empty cell is an empty list, while every delimiter splits off an item, empty or not
    let split_expr = column_expr.clone().str().split(lit(delimiter));
    let items_expr = when(column_expr.eq(lit("")))
        .then(split_expr.clone().list().head(lit(0)))
        .otherwise(split_expr);
    let item_type = match column.property.item_type {
        None | Some(ListItemType::String) => return items_expr,
        Some(item_type) => item_type,
    };

    let list_expr = items_expr.list().eval(cast_item(item_type, col("")));

    // An unparsable item becomes null rather than failing, so a null item marks the cell
    when(count_items(list_expr.clone(), col("").is_null()).gt(lit(0)))
        .then(lit(NULL))
        .otherwise(list_expr)
}

pub fn stringify_list_column(column: &ListColumn, column_expr: Expr) -> Expr {
    column_expr
        .cast(DataType::List(Box::new(DataType::String)))
        .list()
        .join(lit(get_delimiter(column)), true)
}

fn get_delimiter(column: &ListColumn) -> String {
    column
        .property
        .delimiter
        .clone()
        .filter(|delimiter| !delimiter.is_empty())
        .unwrap_or_else(|| ",".to_string())
}

fn cast_item(item_type: ListItemType, item_expr: Expr) -> Expr {
    match item_type {
        ListItemType::Integer => parse_integer_text(item_expr),
        ListItemType::Number => parse_number_text(item_expr),
        ListItemType::Boolean => when(item_expr.clone().is_in(
            lit(Series::new("".into(), TRUE_VALUES)).implode(true),
            false,
        ))
        .then(lit(true))
        .when(item_expr.is_in(
            lit(Series::new("".into(), FALSE_VALUES)).implode(true),
            false,
        ))
        .then(lit(false))
        .otherwise(lit(NULL).cast(DataType::Boolean)),
        ListItemType::DateTime => parse_date_time_text(item_expr),
        ListItemType::LocalDateTime => parse_local_date_time_text(item_expr),
        ListItemType::Date => parse_date_text(item_expr),
        ListItemType::Time => parse_time_text(item_expr),
        ListItemType::LocalTime => parse_local_time_text(item_expr),
        ListItemType::String => item_expr,
    }
}

fn count_items(list_expr: Expr, predicate_expr: Expr) -> Expr {
    list_expr
        .list()
        .eval(predicate_expr.cast(DataType::UInt32))
        .list()
        .sum()
}
