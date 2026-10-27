use fairspec_metadata::{ListColumn, ListItemType};
use polars::prelude::*;

use super::temporal::{create_strptime_options, get_utc_datetime_type};

const TRUE_VALUES: [&str; 4] = ["true", "True", "TRUE", "1"];
const FALSE_VALUES: [&str; 4] = ["false", "False", "FALSE", "0"];
const DATE_PATTERN: &str = r"^\d{4}-\d{2}-\d{2}$";

pub fn parse_list_column(column: &ListColumn, column_expr: Expr) -> Expr {
    let delimiter = get_delimiter(column);
    let items_expr = column_expr.str().split(lit(delimiter));
    let item_type = match column.property.item_type {
        None | Some(ListItemType::String) => return items_expr,
        Some(item_type) => item_type,
    };

    let list_expr = items_expr
        .clone()
        .list()
        .eval(parse_item(item_type, col("")));

    // An unparsable item becomes null rather than failing, and an empty one is null
    // by design, so a list holding more nulls than empty items had an invalid item
    when(
        count_items(list_expr.clone(), col("").is_null())
            .gt(count_items(items_expr, col("").eq(lit("")))),
    )
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

fn parse_item(item_type: ListItemType, item_expr: Expr) -> Expr {
    when(item_expr.clone().eq(lit("")))
        .then(lit(NULL))
        .otherwise(cast_item(item_type, item_expr))
}

fn cast_item(item_type: ListItemType, item_expr: Expr) -> Expr {
    match item_type {
        ListItemType::Integer => item_expr.cast(DataType::Int64),
        ListItemType::Number => item_expr.cast(DataType::Float64),
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
        ListItemType::DateTime => item_expr.str().strptime(
            get_utc_datetime_type(),
            create_strptime_options("%Y-%m-%dT%H:%M:%S%.f%#z"),
            lit("raise"),
        ),
        ListItemType::LocalDateTime => item_expr.str().strptime(
            DataType::Datetime(TimeUnit::Microseconds, None),
            create_strptime_options("%Y-%m-%dT%H:%M:%S%.f"),
            lit("raise"),
        ),
        ListItemType::Date => item_expr
            .str()
            .extract(lit(DATE_PATTERN), 0)
            .str()
            .strptime(
                DataType::Date,
                create_strptime_options("%Y-%m-%d"),
                lit("raise"),
            ),
        ListItemType::Time => concat_str([lit("1970-01-01T"), item_expr], "", false)
            .str()
            .strptime(
                get_utc_datetime_type(),
                create_strptime_options("%Y-%m-%dT%H:%M:%S%.f%#z"),
                lit("raise"),
            )
            .cast(DataType::Time),
        ListItemType::LocalTime => item_expr.str().strptime(
            DataType::Time,
            create_strptime_options("%H:%M:%S%.f"),
            lit("raise"),
        ),
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
