use polars::prelude::*;

use crate::actions::table::query::{query_table, query_tables};

fn collect_rows(table: LazyFrame) -> Vec<Vec<String>> {
    let frame = table.collect().unwrap();
    (0..frame.height())
        .map(|row| {
            frame
                .columns()
                .iter()
                .map(|column| column.get(row).unwrap().to_string())
                .collect()
        })
        .collect()
}

#[test]
fn test_select_all() {
    let table = df!("name" => ["Alice", "Bob"], "age" => [30i64, 25])
        .unwrap()
        .lazy();

    let result = query_table(table, "SELECT * FROM self").unwrap();

    assert_eq!(
        collect_rows(result),
        [["\"Alice\"", "30"], ["\"Bob\"", "25"]]
    );
}

#[test]
fn test_select_with_filter() {
    let table = df!("name" => ["Alice", "Bob"], "age" => [30i64, 25])
        .unwrap()
        .lazy();

    let result = query_table(table, "SELECT name FROM self WHERE age > 26").unwrap();

    assert_eq!(collect_rows(result), [["\"Alice\""]]);
}

#[test]
fn test_returns_lazy_frame() {
    let table = df!("x" => [1i64]).unwrap().lazy();

    let result: LazyFrame = query_table(table, "SELECT * FROM self").unwrap();

    assert_eq!(result.collect().unwrap().height(), 1);
}

#[test]
fn test_join_tables_by_name() {
    let users = df!("id" => [1i64, 2], "name" => ["Alice", "Bob"])
        .unwrap()
        .lazy();
    let posts = df!("user_id" => [2i64, 1, 2], "title" => ["a", "b", "c"])
        .unwrap()
        .lazy();

    let result = query_tables(
        vec![("users".to_string(), users), ("posts".to_string(), posts)],
        "SELECT name, COUNT(*) AS n FROM posts JOIN users ON posts.user_id = users.id \
         GROUP BY name ORDER BY name",
    )
    .unwrap();

    let frame = result.collect().unwrap();
    let names: Vec<String> = frame
        .get_column_names()
        .iter()
        .map(|name| name.to_string())
        .collect();
    assert_eq!(names, ["name", "n"]);
    assert_eq!(
        collect_rows(frame.lazy()),
        [["\"Alice\"", "1"], ["\"Bob\"", "2"]]
    );
}
