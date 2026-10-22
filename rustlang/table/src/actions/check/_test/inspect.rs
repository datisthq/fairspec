use fairspec_metadata::{TableError, TableSchema};
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::check::inspect::inspect_table_checks;
use crate::actions::table::inspect::inspect_table;
use crate::models::check::{TableCheck, TableCheckCondition};

fn create_table() -> LazyFrame {
    df!(
        "id" => [1i64, 2, 3],
        "zip" => ["a", "a", "b"],
        "start" => [1i64, 3, 1],
        "end" => [2i64, 2, 5],
        "price" => [Some(10i64), None, Some(-1)],
    )
    .unwrap()
    .lazy()
}

fn create_check(name: &str, condition: TableCheckCondition) -> TableCheck {
    TableCheck {
        name: name.to_string(),
        condition,
        column_name: None,
        message: None,
    }
}

fn sql(name: &str, condition: &str) -> TableCheck {
    create_check(name, TableCheckCondition::Sql(condition.to_string()))
}

fn expr(name: &str, condition: Expr) -> TableCheck {
    create_check(name, TableCheckCondition::Expr(condition))
}

fn dump(errors: &[TableError]) -> Vec<Value> {
    errors
        .iter()
        .map(|error| serde_json::to_value(error).unwrap())
        .collect()
}

fn inspect(checks: &[TableCheck], max_errors: usize) -> Vec<Value> {
    dump(&inspect_table_checks(create_table(), checks, None, max_errors).unwrap())
}

fn row_check(name: &str, row_number: usize) -> Value {
    json!({"type": "row/check", "checkName": name, "rowNumber": row_number})
}

#[test]
fn test_should_report_rows_failing_a_sql_condition() {
    let errors = inspect(&[sql("end-after-start", "\"end\" >= start")], 100);
    assert_eq!(errors, vec![row_check("end-after-start", 2)]);
}

#[test]
fn test_should_report_rows_failing_a_polars_condition() {
    let errors = inspect(
        &[expr("end-after-start", col("end").gt_eq(col("start")))],
        100,
    );
    assert_eq!(errors, vec![row_check("end-after-start", 2)]);
}

#[test]
fn test_should_pass_rows_where_the_condition_is_null() {
    let errors = inspect(&[sql("positive-price", "price > 0")], 100);
    assert_eq!(errors, vec![row_check("positive-price", 3)]);
}

#[test]
fn test_should_report_a_cell_when_the_check_names_a_column() {
    let mut check = expr("positive-price", col("price").gt(lit(0)));
    check.column_name = Some("price".to_string());
    check.message = Some("Price must be positive".to_string());

    let errors = inspect(&[check], 100);
    assert_eq!(
        errors,
        vec![json!({
            "type": "cell/check",
            "checkName": "positive-price",
            "columnName": "price",
            "rowNumber": 3,
            "cell": "-1",
            "message": "Price must be positive",
        })]
    );
}

#[test]
fn test_should_report_the_table_when_an_aggregate_condition_fails() {
    let errors = inspect(
        &[
            sql("sql-size", "count(*) >= 5"),
            expr("polars-size", len().gt_eq(lit(5))),
            sql("sql-sum", "sum(price) > 0"),
            expr("polars-sum", col("price").sum().gt(lit(0))),
        ],
        100,
    );
    assert_eq!(
        errors,
        vec![
            json!({"type": "table/check", "checkName": "sql-size"}),
            json!({"type": "table/check", "checkName": "polars-size"}),
        ]
    );
}

#[test]
fn test_should_check_rows_against_their_group_with_a_window() {
    let errors = inspect(
        &[
            sql("sql-zip", "count(*) OVER (PARTITION BY zip) >= 2"),
            expr(
                "polars-zip",
                len().over([col("zip")]).unwrap().gt_eq(lit(2)),
            ),
        ],
        100,
    );
    assert_eq!(
        errors,
        vec![row_check("sql-zip", 3), row_check("polars-zip", 3)]
    );
}

#[test]
fn test_should_report_checks_that_cannot_be_evaluated() {
    let mut cell_column = sql("cell-column", "price > 0");
    cell_column.column_name = Some("missing".to_string());

    let errors = inspect(
        &[
            sql("sql-column", "missing > 1"),
            expr("polars-column", col("missing").gt(lit(1))),
            sql("sql-syntax", "price >"),
            sql("not-boolean", "price + 1"),
            cell_column,
            sql("valid", "price > 0"),
        ],
        100,
    );

    let summary: Vec<(Value, Value)> = errors
        .iter()
        .map(|error| (error["type"].clone(), error["checkName"].clone()))
        .collect();
    assert_eq!(
        summary,
        vec![
            (json!("check"), json!("sql-column")),
            (json!("check"), json!("polars-column")),
            (json!("check"), json!("sql-syntax")),
            (json!("check"), json!("not-boolean")),
            (json!("check"), json!("cell-column")),
            (json!("row/check"), json!("valid")),
        ]
    );
    assert_eq!(
        errors[3],
        json!({
            "type": "check",
            "checkName": "not-boolean",
            "message": "Condition is Int64 instead of Boolean",
        })
    );
    assert_eq!(
        errors[4],
        json!({
            "type": "check",
            "checkName": "cell-column",
            "message": "Column \"missing\" is not found",
        })
    );
}

#[test]
fn test_should_stop_at_the_maximum_number_of_failing_rows() {
    let errors = inspect(&[expr("never", lit(false).and(col("id").is_not_null()))], 2);
    assert_eq!(errors, vec![row_check("never", 1), row_check("never", 2)]);
}

#[test]
fn test_should_run_conditions_on_values_typed_by_the_schema() {
    let source = df!("id" => ["1", "2", "3"], "amount" => ["9", "10", "bad"])
        .unwrap()
        .lazy();
    let schema: TableSchema = serde_json::from_value(json!({
        "properties": {"id": {"type": "integer"}, "amount": {"type": "integer"}},
    }))
    .unwrap();
    let mut check = sql("small-amount", "amount < 10");
    check.column_name = Some("amount".to_string());

    let errors = inspect_table(source, Some(&schema), Some(&[check]), None, None, None).unwrap();
    assert_eq!(
        dump(&errors),
        vec![
            json!({
                "type": "cell/type",
                "columnName": "amount",
                "columnType": "integer",
                "rowNumber": 3,
                "cell": "bad",
            }),
            json!({
                "type": "cell/check",
                "checkName": "small-amount",
                "columnName": "amount",
                "rowNumber": 2,
                "cell": "10",
            }),
        ]
    );
}

#[test]
fn test_should_run_checks_without_a_schema() {
    let checks = [sql("end-after-start", "\"end\" >= start")];
    let errors = inspect_table(create_table(), None, Some(&checks), None, None, None).unwrap();
    assert_eq!(dump(&errors), vec![row_check("end-after-start", 2)]);
}
