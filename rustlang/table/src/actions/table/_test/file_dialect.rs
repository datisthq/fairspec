use fairspec_metadata::FileDialect;
use polars::prelude::*;
use serde_json::{Value, json};

use crate::actions::table::file_dialect::{join_header_rows, skip_comment_rows};
use crate::helpers::value::get_json_value;

fn dialect(value: Value) -> FileDialect {
    let mut value = value;
    value["format"] = json!("csv");
    serde_json::from_value(value).unwrap()
}

fn collect(table: LazyFrame) -> (Vec<String>, Vec<Value>) {
    let frame = table.collect().unwrap();
    let columns = frame
        .get_column_names()
        .iter()
        .map(|name| name.to_string())
        .collect();
    let rows = (0..frame.height())
        .map(|index| {
            let row = frame.get_row(index).unwrap();
            Value::Array(row.0.iter().map(get_json_value).collect())
        })
        .collect();
    (columns, rows)
}

mod join_header_rows {
    use super::*;

    #[test]
    fn test_should_join_two_header_rows_with_default_space_separator() {
        let table = df!(
            "column_1" => ["first", "name", "Alice", "Bob"],
            "column_2" => ["last", "name", "Smith", "Jones"],
            "column_3" => ["contact", "email", "alice@example.com", "bob@example.com"],
        )
        .unwrap()
        .lazy();
        let result = join_header_rows(table, &dialect(json!({"headerRows": [1, 2]}))).unwrap();
        let (columns, rows) = collect(result);
        assert_eq!(columns, ["first name", "last name", "contact email"]);
        assert_eq!(
            rows,
            [
                json!(["Alice", "Smith", "alice@example.com"]),
                json!(["Bob", "Jones", "bob@example.com"]),
            ]
        );
    }

    #[test]
    fn test_should_join_two_header_rows_with_custom_separator() {
        let table = df!(
            "column_1" => ["user", "first", "Alice"],
            "column_2" => ["user", "last", "Smith"],
            "column_3" => ["meta", "created", "2023-01-01"],
        )
        .unwrap()
        .lazy();
        let result = join_header_rows(
            table,
            &dialect(json!({"headerRows": [1, 2], "headerJoin": "_"})),
        )
        .unwrap();
        let (columns, rows) = collect(result);
        assert_eq!(columns, ["user_first", "user_last", "meta_created"]);
        assert_eq!(rows, [json!(["Alice", "Smith", "2023-01-01"])]);
    }

    #[test]
    fn test_should_return_table_unchanged_when_only_one_header_row() {
        let table = df!(
            "name" => ["Alice", "Bob"],
            "age" => [30i64, 25],
            "city" => ["NYC", "LA"],
        )
        .unwrap()
        .lazy();
        let result = join_header_rows(table, &dialect(json!({"headerRows": [1]}))).unwrap();
        let (columns, rows) = collect(result);
        assert_eq!(columns, ["name", "age", "city"]);
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_should_return_table_unchanged_when_no_header_rows() {
        let table = df!(
            "field1" => ["Alice", "Bob"],
            "field2" => [30i64, 25],
            "field3" => ["NYC", "LA"],
        )
        .unwrap()
        .lazy();
        let result = join_header_rows(table, &dialect(json!({}))).unwrap();
        let (columns, rows) = collect(result);
        assert_eq!(columns, ["field1", "field2", "field3"]);
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn test_should_join_three_header_rows_after_skipped_rows() {
        let table = df!(
            "column_1" => ["person", "user", "first", "Alice"],
            "column_2" => ["person", "user", "last", "Smith"],
            "column_3" => ["location", "address", "city", "NYC"],
        )
        .unwrap()
        .lazy();
        let result = join_header_rows(table, &dialect(json!({"headerRows": [2, 3, 4]}))).unwrap();
        let (columns, rows) = collect(result);
        assert_eq!(
            columns,
            [
                "person user first",
                "person user last",
                "location address city"
            ]
        );
        assert_eq!(rows, [json!(["Alice", "Smith", "NYC"])]);
    }

    #[test]
    fn test_should_carry_a_label_forward_across_empty_header_cells() {
        let table = df!(
            "column_1" => [Some("person"), Some("first"), Some("Alice")],
            "column_2" => [None, Some("last"), Some("Smith")],
            "column_3" => [Some("location"), Some("city"), Some("NYC")],
        )
        .unwrap()
        .lazy();
        let result = join_header_rows(table, &dialect(json!({"headerRows": [1, 2]}))).unwrap();
        let (columns, rows) = collect(result);
        assert_eq!(columns, ["person first", "person last", "location city"]);
        assert_eq!(rows, [json!(["Alice", "Smith", "NYC"])]);
    }

    #[test]
    fn test_should_drop_the_other_header_rows_with_column_names() {
        let table = df!("key" => ["id", "1"], "label" => ["name", "apple"])
            .unwrap()
            .lazy();
        let result = join_header_rows(
            table,
            &dialect(json!({"headerRows": [1, 2], "columnNames": ["key", "label"]})),
        )
        .unwrap();
        let (columns, rows) = collect(result);
        assert_eq!(columns, ["key", "label"]);
        assert_eq!(rows, [json!(["1", "apple"])]);
    }
}

mod skip_comment_rows {
    use super::*;

    #[test]
    fn test_should_skip_comment_rows_by_row_number() {
        let table = df!(
            "name" => ["Alice", "# Comment", "Bob", "Charlie"],
            "age" => [30i64, 0, 25, 35],
            "city" => ["NYC", "ignored", "LA", "SF"],
        )
        .unwrap()
        .lazy();
        let result = skip_comment_rows(
            table,
            &dialect(json!({"commentRows": [2], "headerRows": false})),
        );
        let (_, rows) = collect(result);
        assert_eq!(
            rows,
            [
                json!(["Alice", 30, "NYC"]),
                json!(["Bob", 25, "LA"]),
                json!(["Charlie", 35, "SF"]),
            ]
        );
    }

    #[test]
    fn test_should_skip_multiple_comment_rows() {
        let table = df!(
            "name" => ["Alice", "# Comment 1", "Bob", "# Comment 2", "Charlie"],
            "age" => [30i64, 0, 25, 0, 35],
            "city" => ["NYC", "ignored", "LA", "ignored", "SF"],
        )
        .unwrap()
        .lazy();
        let result = skip_comment_rows(
            table,
            &dialect(json!({"commentRows": [2, 4], "headerRows": false})),
        );
        let (_, rows) = collect(result);
        assert_eq!(
            rows,
            [
                json!(["Alice", 30, "NYC"]),
                json!(["Bob", 25, "LA"]),
                json!(["Charlie", 35, "SF"]),
            ]
        );
    }

    #[test]
    fn test_should_return_table_unchanged_when_no_comment_rows_specified() {
        let table = df!(
            "name" => ["Alice", "Bob", "Charlie"],
            "age" => [30i64, 25, 35],
            "city" => ["NYC", "LA", "SF"],
        )
        .unwrap()
        .lazy();
        let result = skip_comment_rows(table, &dialect(json!({})));
        let (columns, rows) = collect(result);
        assert_eq!(rows.len(), 3);
        assert_eq!(columns, ["name", "age", "city"]);
    }

    #[test]
    fn test_should_skip_rows_after_header_when_header_rows_specified() {
        let table = df!(
            "col1" => ["name", "Alice", "# Comment", "Bob"],
            "col2" => ["age", "30", "-1", "25"],
            "col3" => ["city", "NYC", "ignored", "LA"],
        )
        .unwrap()
        .lazy();
        let result = skip_comment_rows(
            table,
            &dialect(json!({"headerRows": [2], "commentRows": [5]})),
        );
        let (_, rows) = collect(result);
        assert_eq!(
            rows,
            [
                json!(["name", "age", "city"]),
                json!(["Alice", "30", "NYC"]),
                json!(["Bob", "25", "LA"]),
            ]
        );
    }

    #[test]
    fn test_should_handle_comment_rows_at_the_beginning() {
        let table = df!(
            "name" => ["# Skip this", "Alice", "Bob"],
            "age" => [0i64, 30, 25],
            "city" => ["ignored", "NYC", "LA"],
        )
        .unwrap()
        .lazy();
        let result = skip_comment_rows(
            table,
            &dialect(json!({"commentRows": [1], "headerRows": false})),
        );
        let (_, rows) = collect(result);
        assert_eq!(
            rows,
            [json!(["Alice", 30, "NYC"]), json!(["Bob", 25, "LA"])]
        );
    }

    #[test]
    fn test_should_handle_comment_rows_at_the_end() {
        let table = df!(
            "name" => ["Alice", "Bob", "# Footer comment"],
            "age" => [30i64, 25, 0],
            "city" => ["NYC", "LA", "ignored"],
        )
        .unwrap()
        .lazy();
        let result = skip_comment_rows(
            table,
            &dialect(json!({"commentRows": [3], "headerRows": false})),
        );
        let (_, rows) = collect(result);
        assert_eq!(
            rows,
            [json!(["Alice", 30, "NYC"]), json!(["Bob", 25, "LA"])]
        );
    }

    #[test]
    fn test_should_handle_multiple_header_rows_with_comment_rows() {
        let table = df!(
            "col1" => ["person", "first", "Alice", "# Comment", "Bob"],
            "col2" => ["person", "last", "Smith", "ignored", "Jones"],
            "col3" => ["location", "city", "NYC", "ignored", "LA"],
        )
        .unwrap()
        .lazy();
        let result = skip_comment_rows(
            table,
            &dialect(json!({"headerRows": [2, 3], "commentRows": [7]})),
        );
        let (_, rows) = collect(result);
        assert_eq!(
            rows,
            [
                json!(["person", "person", "location"]),
                json!(["first", "last", "city"]),
                json!(["Alice", "Smith", "NYC"]),
                json!(["Bob", "Jones", "LA"]),
            ]
        );
    }
}
